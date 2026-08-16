//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 471/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk471(t322: f64, t1013: f64, t833: f64, t829: f64, t1300: f64, t2394: f64, t327: f64, t834: f64, t330: f64, t1018: f64, t837: f64, t2393: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t2397 = t1013 * t833;
    let t2400 = t1013 * t829;
    let t2405 = -0.64e0_f64 * t2394 * t327 - 0.128e1_f64 * t2397 * t829 - 0.128e1_f64 * t1300 * t2400 - 0.64e0_f64 * t834 * t2394;
    let t2406 = t2405 * t330;
    let t2407 = t1018 * t837;
    let t2408 = t2407 * t330;
    let t2410 = piecewise3(t332, 0.0_f64, t2393);
    (t2397, t2400, t2405, t2406, t2408, t2410)
}
