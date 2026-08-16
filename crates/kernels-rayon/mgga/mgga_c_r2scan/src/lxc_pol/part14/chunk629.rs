//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 629/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk629(t322: f64, t1120: f64, t829: f64, t1300: f64, t327: f64, t3506: f64, t3509: f64, t834: f64, t330: f64, t1125: f64, t837: f64, t3505: f64) -> (f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t3512 = t1120 * t829;
    let t3517 = -0.64e0_f64 * t3506 * t327 - 0.128e1_f64 * t3509 * t829 - 0.128e1_f64 * t1300 * t3512 - 0.64e0_f64 * t834 * t3506;
    let t3518 = t3517 * t330;
    let t3519 = t1125 * t837;
    let t3520 = t3519 * t330;
    let t3522 = piecewise3(t332, 0.0_f64, t3505);
    (t3517, t3518, t3520, t3522)
}
