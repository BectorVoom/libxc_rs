//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 517/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk517(t2257: f64, t705: f64, t2187: f64, t2190: f64, t2193: f64, t2197: f64, t2199: f64, t2202: f64, t697: f64, t164: f64, t172: f64, t123: f64, t147: f64, t2192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2258 = t2257 * t705;
    let t2267 = -0.78438333333333333333e0_f64 * t2187 + 0.15687666666666666667e1_f64 * t2190 + 0.68863333333333333333e0_f64 * t2193 + 0.14025833333333333333e0_f64 * t2197 + 0.28051666666666666667e0_f64 * t2199 + 0.17365833333333333333e0_f64 * t2202;
    let t2268 = t2267 * t705;
    let t2271 = t697 * t697;
    let t2272 = 1.0_f64 / t2271;
    let t2273 = t164 * t2272;
    let t2274 = t172 * t172;
    let t2275 = 1.0_f64 / t2274;
    let t2276 = t2257 * t2275;
    let t2281 = 0.14764627977777777777e-2_f64 * t123 * t2192 * t147;
    (t2258, t2267, t2268, t2271, t2272, t2273, t2274, t2275, t2276, t2281)
}
