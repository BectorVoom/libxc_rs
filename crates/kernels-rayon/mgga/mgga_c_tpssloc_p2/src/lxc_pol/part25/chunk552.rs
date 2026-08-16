//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 552/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk552(t1124: f64, t1128: f64, t1127: f64, t432: f64, t427: f64, t1136: f64, t1137: f64, t3236: f64, t3293: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3288: f64, t3290: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3327 = t1124 * t1128;
    let t3330 = t1127 * t432;
    let t3331 = 1.0_f64 / t3330;
    let t3332 = t427 * t3331;
    let t3333 = t1136 * t1136;
    let t3334 = t3333 * t1137;
    let t3339 = 0.68863333333333333333e0_f64 * t3236;
    let t3346 = 0.17365833333333333333e0_f64 * t3293;
    let t3351 = -0.17648625e1_f64 * t3272 + 0.3529725e1_f64 * t3280 + t3339 - 0.34431666666666666666e0_f64 * t3238 - 0.34431666666666666667e0_f64 * t3245 + 0.103295e1_f64 * t3250 + 0.516475e0_f64 * t3254 + 0.31558125e0_f64 * t3288 + 0.6311625e0_f64 * t3290 + t3346 - 0.13892666666666666667e0_f64 * t3295 - 0.34731666666666666667e-1_f64 * t3299 + 0.20839e0_f64 * t3302 + 0.104195e0_f64 * t3305;
    (t3327, t3331, t3332, t3333, t3334, t3351)
}
