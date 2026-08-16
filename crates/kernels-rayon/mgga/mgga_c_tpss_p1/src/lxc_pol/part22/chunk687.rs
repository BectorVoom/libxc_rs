//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 687/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk687(t30: f64, t33: f64, t3274: f64, t3275: f64, t3273: f64, t2331: f64, t497: f64, t489: f64, t502: f64, t1991: f64, t3218: f64, t490: f64, t504: f64, t2829: f64, t3226: f64, t493: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3276 = t3274 * t3275;
    let t3277 = t3273 * t3276;
    let t3280 = t497 * t2331;
    let t3281 = t489 * t3280;
    let t3282 = 1.0_f64 / t502;
    let t3288 = piecewise3(t31, 0.0_f64, 4.0_f64 / 9.0_f64 * t3282 * t3218 + 4.0_f64 / 3.0_f64 * t490 * t1991);
    let t3289 = 1.0_f64 / t504;
    let t3295 = piecewise3(t34, 0.0_f64, 4.0_f64 / 9.0_f64 * t3289 * t3226 + 4.0_f64 / 3.0_f64 * t493 * t2829);
    (t3277, t3280, t3281, t3282, t3288, t3289, t3295)
}
