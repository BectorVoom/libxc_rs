//! Error function (erf) and complementary error function (erfc) implementations.
//!
//! Cephes/libm-style piecewise rational approximation with full f64 precision.
//! Uses nested `select()` calls for branchless region dispatch per D-05/D-06.
//!
//! Coefficients are taken from the Sun Microsystems libm implementation (fdlibm),
//! which is also the basis for Rust's libm crate.


// ============================================================================
// Cephes/fdlibm erf coefficients
// ============================================================================

// erx = erf(1) - 1 (for the 0.84375..1.25 region)
const ERX: f64 = 8.45062911510467529297e-01;

// Coefficients for approximation to erf on [0, 0.84375]
// erf(x) = x + x * R(x^2) where R = pp/qq
const PP0: f64 = 1.28379167095512558561e-01;
const PP1: f64 = -3.25042107247001499370e-01;
const PP2: f64 = -2.84817495755985104766e-02;
const PP3: f64 = -5.77027029648944159157e-03;
const PP4: f64 = -2.37630166566501626084e-05;
const QQ1: f64 = 3.97917223959155352819e-01;
const QQ2: f64 = 6.50222499887672944485e-02;
const QQ3: f64 = 5.08130628187576562776e-03;
const QQ4: f64 = 1.32494738004321644526e-04;
const QQ5: f64 = -3.96022827877536812320e-06;

// Coefficients for approximation to erfc on [0.84375, 1.25]
const PA0: f64 = -2.36211856075265944077e-03;
const PA1: f64 = 4.14856118683748331666e-01;
const PA2: f64 = -3.72207876035701323847e-01;
const PA3: f64 = 3.18346619901161753674e-01;
const PA4: f64 = -1.10894694282396677476e-01;
const PA5: f64 = 3.54783043195201877747e-02;
const PA6: f64 = -2.16637559983254089680e-03;
const QA1: f64 = 1.06420880400844228286e-01;
const QA2: f64 = 5.40397917702171048937e-01;
const QA3: f64 = 7.18286544141962539399e-02;
const QA4: f64 = 1.26171219808761642112e-01;
const QA5: f64 = 1.36370839120290507362e-02;
const QA6: f64 = 1.19844998467991074170e-02;

// Coefficients for approximation to erfc on [1.25, 2.857142857]
const RA0: f64 = -9.86494403484714822705e-03;
const RA1: f64 = -6.93858572707181764372e-01;
const RA2: f64 = -1.05586262253232909814e+01;
const RA3: f64 = -6.23753324503260060396e+01;
const RA4: f64 = -1.62396669462573071767e+02;
const RA5: f64 = -1.84605092906711035994e+02;
const RA6: f64 = -8.12874355063065934246e+01;
const RA7: f64 = -9.81432934416914548592e+00;
const SA1: f64 = 1.96512716674392571292e+01;
const SA2: f64 = 1.37657754143519702237e+02;
const SA3: f64 = 4.34565877475229228608e+02;
const SA4: f64 = 6.45387271733267880594e+02;
const SA5: f64 = 4.29008140027567833386e+02;
const SA6: f64 = 1.08635005541779435134e+02;
const SA7: f64 = 6.57024977031928170135e+00;
const SA8: f64 = -6.04244152148580987438e-02;

// Coefficients for approximation to erfc on [2.857142857, 6]
const RB0: f64 = -9.86494292470009928597e-03;
const RB1: f64 = -7.99283237680523006574e-01;
const RB2: f64 = -1.77579549177547519889e+01;
const RB3: f64 = -1.60636384855557935030e+02;
const RB4: f64 = -6.37566443368389085394e+02;
const RB5: f64 = -1.02509513161107724954e+03;
const RB6: f64 = -4.83519191608651397019e+02;
const SB1: f64 = 3.03380607875625778203e+01;
const SB2: f64 = 3.25792512996573918826e+02;
const SB3: f64 = 1.53672958608443695994e+03;
const SB4: f64 = 3.19985821950859553908e+03;
const SB5: f64 = 2.55305040643316442583e+03;
const SB6: f64 = 4.74528541206955367215e+02;
const SB7: f64 = -2.24409524465858183362e+01;

/// Compute the error function erf(x) with full f64 precision.
///
/// Uses Cephes/libm piecewise rational approximation with branchless select.
/// Accuracy: relative error <= 1e-15 across [-6, 6].
pub fn erf_approx(x: f64) -> f64 {
    let abs_x = f64::abs(x);
    let sign = (if x < 0.0_f64 { -1.0_f64 } else { 1.0_f64 });

    // Region 1: |x| < 0.84375
    let x2 = x * x;
    let pp = (PP0 as f64) + x2 * ((PP1 as f64) + x2 * ((PP2 as f64) + x2 * ((PP3 as f64) + x2 * (PP4 as f64))));
    let qq = 1.0_f64 + x2 * ((QQ1 as f64) + x2 * ((QQ2 as f64) + x2 * ((QQ3 as f64) + x2 * ((QQ4 as f64) + x2 * (QQ5 as f64)))));
    let r1 = x + x * (pp / qq);

    // Region 2: 0.84375 <= |x| < 1.25
    let s = abs_x - 1.0_f64;
    let pa = (PA0 as f64) + s * ((PA1 as f64) + s * ((PA2 as f64) + s * ((PA3 as f64) + s * ((PA4 as f64) + s * ((PA5 as f64) + s * (PA6 as f64))))));
    let qa = 1.0_f64 + s * ((QA1 as f64) + s * ((QA2 as f64) + s * ((QA3 as f64) + s * ((QA4 as f64) + s * ((QA5 as f64) + s * (QA6 as f64))))));
    let r2 = sign * ((ERX as f64) + pa / qa);

    // Region 3: 1.25 <= |x| < 2.857142857
    // Formula: erfc(x) = exp(-x_hi^2 - 0.5625) * exp(-x_lo*(x+x_hi) + R/S) / x
    // High-precision exp trick (fdlibm): split x into x_hi (truncated) and x_lo = x - x_hi
    // to avoid precision loss in exp(-x^2) for moderate x values.
    // Truncate to ~20 mantissa bits (matches fdlibm SET_LOW_WORD(z,0) which zeros
    // 32 low bits). x_hi^2 then has ~40 mantissa bits, fitting exactly in f64's 52-bit
    // mantissa, making exp(-x_hi^2) exact to full precision.
    let x_hi = f64::floor(abs_x * 1048576.0_f64) / 1048576.0_f64;
    let x_lo = abs_x - x_hi;
    let s3 = 1.0_f64 / (abs_x * abs_x);
    let ra = (RA0 as f64) + s3 * ((RA1 as f64) + s3 * ((RA2 as f64) + s3 * ((RA3 as f64) + s3 * ((RA4 as f64) + s3 * ((RA5 as f64) + s3 * ((RA6 as f64) + s3 * (RA7 as f64)))))));
    let sa = 1.0_f64 + s3 * ((SA1 as f64) + s3 * ((SA2 as f64) + s3 * ((SA3 as f64) + s3 * ((SA4 as f64) + s3 * ((SA5 as f64) + s3 * ((SA6 as f64) + s3 * ((SA7 as f64) + s3 * (SA8 as f64))))))));
    let r_over_s_3 = ra / sa;
    let erfc3 = f64::exp(-x_hi * x_hi - 0.5625_f64) * f64::exp(-x_lo * (abs_x + x_hi) + r_over_s_3) / abs_x;
    let r3 = sign * (1.0_f64 - erfc3);

    // Region 4: 2.857142857 <= |x| < 6
    let s4 = 1.0_f64 / (abs_x * abs_x);
    let rb = (RB0 as f64) + s4 * ((RB1 as f64) + s4 * ((RB2 as f64) + s4 * ((RB3 as f64) + s4 * ((RB4 as f64) + s4 * ((RB5 as f64) + s4 * (RB6 as f64))))));
    let sb = 1.0_f64 + s4 * ((SB1 as f64) + s4 * ((SB2 as f64) + s4 * ((SB3 as f64) + s4 * ((SB4 as f64) + s4 * ((SB5 as f64) + s4 * ((SB6 as f64) + s4 * (SB7 as f64)))))));
    let r_over_s_4 = rb / sb;
    let erfc4 = f64::exp(-x_hi * x_hi - 0.5625_f64) * f64::exp(-x_lo * (abs_x + x_hi) + r_over_s_4) / abs_x;
    let r4 = sign * (1.0_f64 - erfc4);

    // Region 5: |x| >= 6
    let r5 = sign * 1.0_f64;

    // Branchless region selection using nested select
    (if abs_x < 0.84375_f64 { r1 } else { (if abs_x < 1.25_f64 { r2 } else { (if abs_x < 2.857142857_f64 { r3 } else { (if abs_x < 6.0_f64 { r4 } else { r5 }) }) }) })
}

/// Backward-compatible alias for generated kernels that still reference the
/// older CubeCL-facing helper name.
pub fn erf_cube(x: f64) -> f64 {
    erf_approx(x)
}

/// Compute the complementary error function erfc(x) = 1 - erf(x).
///
/// Uses region-specific direct computation to avoid cancellation.
/// Uses fdlibm high-precision exp trick (x split into hi/lo parts) for regions 3-4.
/// Accuracy: relative error < 5e-11 across [-6, 6] (limited by CubeCL branchless eval
/// near region 3/4 boundary; most of the domain achieves < 1e-14).
pub fn erfc_approx(x: f64) -> f64 {
    let abs_x = f64::abs(x);

    // Region 1: |x| < 0.84375 -> erfc = 1 - erf(x), no severe cancellation
    let x2 = x * x;
    let pp = (PP0 as f64) + x2 * ((PP1 as f64) + x2 * ((PP2 as f64) + x2 * ((PP3 as f64) + x2 * (PP4 as f64))));
    let qq = 1.0_f64 + x2 * ((QQ1 as f64) + x2 * ((QQ2 as f64) + x2 * ((QQ3 as f64) + x2 * ((QQ4 as f64) + x2 * (QQ5 as f64)))));
    let erfc1 = 1.0_f64 - (x + x * (pp / qq));

    // Region 2: 0.84375 <= |x| < 1.25
    let s = abs_x - 1.0_f64;
    let pa = (PA0 as f64) + s * ((PA1 as f64) + s * ((PA2 as f64) + s * ((PA3 as f64) + s * ((PA4 as f64) + s * ((PA5 as f64) + s * (PA6 as f64))))));
    let qa = 1.0_f64 + s * ((QA1 as f64) + s * ((QA2 as f64) + s * ((QA3 as f64) + s * ((QA4 as f64) + s * ((QA5 as f64) + s * (QA6 as f64))))));
    // erfc(|x|) = (1 - ERX) - pa/qa; for negative x: erfc = 1 + ERX + pa/qa
    let erfc2 = (if x < 0.0_f64 { 1.0_f64 + (ERX as f64) + pa / qa } else { (1.0_f64 - (ERX as f64)) - pa / qa });

    // Region 3: 1.25 <= |x| < 2.857142857
    // erfc(x) = exp(-x_hi^2 - 0.5625) * exp(-x_lo*(x+x_hi) + R/S) / x
    // High-precision exp trick: split x to avoid precision loss in exp(-x^2)
    let x_hi_c = f64::floor(abs_x * 1048576.0_f64) / 1048576.0_f64;
    let x_lo_c = abs_x - x_hi_c;
    let s3 = 1.0_f64 / (abs_x * abs_x);
    let ra = (RA0 as f64) + s3 * ((RA1 as f64) + s3 * ((RA2 as f64) + s3 * ((RA3 as f64) + s3 * ((RA4 as f64) + s3 * ((RA5 as f64) + s3 * ((RA6 as f64) + s3 * (RA7 as f64)))))));
    let sa = 1.0_f64 + s3 * ((SA1 as f64) + s3 * ((SA2 as f64) + s3 * ((SA3 as f64) + s3 * ((SA4 as f64) + s3 * ((SA5 as f64) + s3 * ((SA6 as f64) + s3 * ((SA7 as f64) + s3 * (SA8 as f64))))))));
    let r_over_s_3 = ra / sa;
    let erfc3_pos = f64::exp(-x_hi_c * x_hi_c - 0.5625_f64) * f64::exp(-x_lo_c * (abs_x + x_hi_c) + r_over_s_3) / abs_x;
    let erfc3 = (if x < 0.0_f64 { 2.0_f64 - erfc3_pos } else { erfc3_pos });

    // Region 4: 2.857142857 <= |x| < 6
    let s4 = 1.0_f64 / (abs_x * abs_x);
    let rb = (RB0 as f64) + s4 * ((RB1 as f64) + s4 * ((RB2 as f64) + s4 * ((RB3 as f64) + s4 * ((RB4 as f64) + s4 * ((RB5 as f64) + s4 * (RB6 as f64))))));
    let sb = 1.0_f64 + s4 * ((SB1 as f64) + s4 * ((SB2 as f64) + s4 * ((SB3 as f64) + s4 * ((SB4 as f64) + s4 * ((SB5 as f64) + s4 * ((SB6 as f64) + s4 * (SB7 as f64)))))));
    let r_over_s_4 = rb / sb;
    let erfc4_pos = f64::exp(-x_hi_c * x_hi_c - 0.5625_f64) * f64::exp(-x_lo_c * (abs_x + x_hi_c) + r_over_s_4) / abs_x;
    let erfc4 = (if x < 0.0_f64 { 2.0_f64 - erfc4_pos } else { erfc4_pos });

    // Region 5: |x| >= 6
    let erfc5 = (if x < 0.0_f64 { 2.0_f64 } else { 0.0_f64 });

    // Branchless region selection
    (if abs_x < 0.84375_f64 { erfc1 } else { (if abs_x < 1.25_f64 { erfc2 } else { (if abs_x < 2.857142857_f64 { erfc3 } else { (if abs_x < 6.0_f64 { erfc4 } else { erfc5 }) }) }) })
}
