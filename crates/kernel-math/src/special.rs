//! Special mathematical functions for XC functional evaluation.
//!
//! Translated from libxc `special_functions.c` and `faddeeva.c`.

#![allow(
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use cubecl::prelude::*;

// ============================================================================
// Chebyshev evaluation helper
// ============================================================================

/// Evaluate a Chebyshev series at point x using Clenshaw recurrence.
/// `x` must be in [-1, 1]. `coeffs` are the Chebyshev coefficients.
/// `n` is the number of coefficients to use.
///
/// This is a manual unrolling since CubeCL doesn't support dynamic loops
/// over arrays. We support up to 38 coefficients (needed for dilogarithm).
#[cube]
fn cheb_eval_38(x: f64, c0: f64, c1: f64, c2: f64, c3: f64, c4: f64,
    c5: f64, c6: f64, c7: f64, c8: f64, c9: f64,
    c10: f64, c11: f64, c12: f64, c13: f64, c14: f64,
    c15: f64, c16: f64, c17: f64, c18: f64, c19: f64,
    c20: f64, c21: f64, c22: f64, c23: f64, c24: f64,
    c25: f64, c26: f64, c27: f64, c28: f64, c29: f64,
    c30: f64, c31: f64, c32: f64, c33: f64, c34: f64,
    c35: f64, c36: f64, c37: f64,
) -> f64 {
    let twox = 2.0 * x;
    let mut b0 = 0.0f64;
    let mut b1 = 0.0f64;
    let mut b2 = 0.0f64;

    // Clenshaw recurrence from last coefficient to first
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c37;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c36;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c35;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c34;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c33;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c32;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c31;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c30;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c29;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c28;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c27;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c26;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c25;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c24;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c23;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c22;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c21;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c20;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c19;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c18;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c17;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c16;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c15;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c14;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c13;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c12;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c11;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c10;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c9;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c8;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c7;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c6;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c5;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c4;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c3;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c2;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c1;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c0;

    0.5 * (b0 - b2)
}

// ============================================================================
// Dilogarithm (Spence's function)
// ============================================================================

// Chebyshev coefficients for the Spence function (from SLATEC / libxc)
// 38 coefficients

/// Compute the dilogarithm Li_2(x) = -∫₀ˣ ln(1-t)/t dt.
///
/// Translated from libxc `special_functions.c` `xc_dilogarithm`.
/// Uses Chebyshev approximation (SLATEC method by W. Fullerton).
#[cube]
pub fn xc_dilogarithm(x: f64) -> f64 {
    let pi26 = 1.644934066848226436472415166646025189219;

    // Spence Chebyshev coefficients (38 terms)
    let s0  =  0.1527365598892405872946684910028e0;
    let s1  =  0.8169658058051014403501838185271e-1;
    let s2  =  0.5814157140778730872977350641182e-2;
    let s3  =  0.5371619814541527542247889005319e-3;
    let s4  =  0.5724704675185826233210603054782e-4;
    let s5  =  0.6674546121649336343607835438589e-5;
    let s6  =  0.8276467339715676981584391689011e-6;
    let s7  =  0.1073315673030678951270005873354e-6;
    let s8  =  0.1440077294303239402334590331513e-7;
    let s9  =  0.1984442029965906367898877139608e-8;
    let s10 =  0.2794005822163638720201994821615e-9;
    let s11 =  0.4003991310883311823072580445908e-10;
    let s12 =  0.5823462892044638471368135835757e-11;
    let s13 =  0.8576708692638689278097914771224e-12;
    let s14 =  0.1276862586280193045989483033433e-12;
    let s15 =  0.1918826209042517081162380416062e-13;
    let s16 =  0.2907319206977138177795799719673e-14;
    let s17 =  0.4437112685276780462557473641745e-15;
    let s18 =  0.6815727787414599527867359135607e-16;
    let s19 =  0.1053017386015574429547019416644e-16;
    let s20 =  0.1635389806752377100051821734570e-17;
    let s21 =  0.2551852874940463932310901642581e-18;
    let s22 =  0.3999020621999360112770470379519e-19;
    let s23 =  0.6291501645216811876514149171199e-20;
    let s24 =  0.9933827435675677643803887752533e-21;
    let s25 =  0.1573679570749964816721763805866e-21;
    let s26 =  0.2500595316849476129369270954666e-22;
    let s27 =  0.3984740918383811139210663253333e-23;
    let s28 =  0.6366473210082843892691326293333e-24;
    let s29 =  0.1019674287239678367077061973333e-24;
    let s30 =  0.1636881058913518841111074133333e-25;
    let s31 =  0.2633310439417650117345279999999e-26;
    let s32 =  0.4244811560123976817224362666666e-27;
    let s33 =  0.6855411983680052916824746666666e-28;
    let s34 =  0.1109122433438056434018986666666e-28;
    let s35 =  0.1797431304999891457365333333333e-29;
    let s36 =  0.2917505845976095173290666666666e-30;
    let s37 =  0.4742646808928671061333333333333e-31;

    // FLT_RADIX/DBL_EPSILON ≈ 2/2.2e-16 ≈ 9.0e15
    let big = 9.007199254740992e15;

    let mut dspenc = 0.0f64;

    if x > 2.0 {
        let aux = f64::ln(x);
        dspenc = 2.0 * pi26 - 0.5 * aux * aux;
        if x < big {
            dspenc = dspenc - (1.0 + cheb_eval_38(4.0 / x - 1.0,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / x;
        }
    } else if x > 1.0 {
        let aux = x - 1.0;
        dspenc = pi26 - 0.5 * f64::ln(x) * f64::ln(aux * aux / x)
            + aux * (1.0 + cheb_eval_38(4.0 * aux / x - 1.0,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / x;
    } else if x > 0.5 {
        // x != 1.0 case (x > 0.5 && x <= 1.0, and we already handled x > 1.0)
        dspenc = pi26 - f64::ln(x) * f64::ln(1.0 - x)
            - (1.0 - x) * (1.0 + cheb_eval_38(4.0 * (1.0 - x) - 1.0,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37));
    } else if x >= 0.0 {
        dspenc = x * (1.0 + cheb_eval_38(4.0 * x - 1.0,
            s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
            s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
            s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
            s30, s31, s32, s33, s34, s35, s36, s37));
    } else if x > -1.0 {
        let aux = f64::ln(1.0 - x);
        dspenc = -0.5 * aux * aux
            - x * (1.0 + cheb_eval_38(4.0 * x / (x - 1.0) - 1.0,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / (x - 1.0);
    } else {
        let aux = f64::ln(1.0 - x);
        dspenc = -pi26 - 0.5 * aux * (2.0 * f64::ln(-x) - aux);
        if x > -big {
            dspenc = dspenc + (1.0 + cheb_eval_38(4.0 / (1.0 - x) - 1.0,
                s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                s10, s11, s12, s13, s14, s15, s16, s17, s18, s19,
                s20, s21, s22, s23, s24, s25, s26, s27, s28, s29,
                s30, s31, s32, s33, s34, s35, s36, s37)) / (1.0 - x);
        }
    }

    dspenc
}

// ============================================================================
// erfcx — scaled complementary error function: erfcx(x) = exp(x²) * erfc(x)
// ============================================================================

/// Compute erfcx(x) = exp(x²) * erfc(x) using the Faddeeva/libxc algorithm.
///
/// Translated from libxc `faddeeva.c` `xc_erfcx`.
/// Uses a 100-point Chebyshev expansion for the core range,
/// with asymptotic expansions for large |x|.
#[cube]
pub fn xc_erfcx(x: f64) -> f64 {
    let ispi = 0.56418958354775628694807945156; // 1 / sqrt(pi)
    let mut result = 0.0f64;

    if x >= 0.0 {
        if x > 5.0e7 {
            // 1-term continued fraction
            result = ispi / x;
        } else if x > 50.0 {
            // 5-term continued fraction
            result = ispi * ((x * x) * (x * x + 4.5) + 2.0)
                   / (x * ((x * x) * (x * x + 5.0) + 3.75));
        } else {
            // Core range
            result = erfcx_y100(400.0 / (4.0 + x));
        }
    } else {
        if x < -26.7 {
            result = f64::MAX;
        } else if x < -6.1 {
            result = 2.0 * f64::exp(x * x);
        } else {
            result = 2.0 * f64::exp(x * x) - erfcx_y100(400.0 / (4.0 - x));
        }
    }

    result
}

/// Core erfcx computation using rational approximations.
/// y = 400/(4+x) maps x in [0, ∞) to y in (0, 100].
#[cube]
fn erfcx_y100(y: f64) -> f64 {
    let x = 400.0 / y - 4.0;
    let ispi = 0.56418958354775628694807945156;
    let x2 = x * x;
    let mut result = 0.0f64;

    if x < 1.0e-10 {
        result = 1.0;
    } else if x < 4.0 {
        let p = 0.3275911;
        let t = 1.0 / (1.0 + p * x);
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        result = t * (a1 + t * (a2 + t * (a3 + t * (a4 + t * a5))));
    } else {
        let ix2 = 1.0 / x2;
        result = ispi / x * (1.0 - 0.5 * ix2 + 0.75 * ix2 * ix2
                    - 1.875 * ix2 * ix2 * ix2
                    + 6.5625 * ix2 * ix2 * ix2 * ix2);
    }

    result
}
