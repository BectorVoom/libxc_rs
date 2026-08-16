//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 813/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk813(t1684: f64, t3005: f64, t3034: f64, t4758: f64, t1211: f64, t5208: f64, t1823: f64, t3574: f64, t13908: f64, t13712: f64, t13714: f64, t4731: f64, t962: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15304 = t1684 * t3005;
    let t15351 = t4758 * t3034;
    let t15362 = t5208 * t1211;
    let t15369 = t1823 * t3574;
    let t15397 = 0.27785333333333333334e0_f64 * t13908;
    let t15411 = 0.22954444444444444444e0_f64 * t13712;
    let t15432 = 0.2283111111111111111e-1_f64 * t13714;
    let t15445 = t4731 * t962;
    (t15304, t15351, t15362, t15369, t15397, t15411, t15432, t15445)
}
