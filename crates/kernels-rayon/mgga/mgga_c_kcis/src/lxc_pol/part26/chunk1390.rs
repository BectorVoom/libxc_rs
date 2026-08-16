//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1390/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1390(t22317: f64, t27494: f64, t17311: f64, t28580: f64, t1555: f64, t29487: f64, t4189: f64, t48044: f64, t8186: f64, t12345: f64, t29427: f64, t5900: f64, t97991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103900 = 2.0_f64 * t27494 * t22317;
    let t103905 = 4.0_f64 * t17311 * t28580;
    let t103909 = 2.0_f64 * t4189 * t29487 * t1555;
    let t103914 = 4.0_f64 * t48044 * t8186;
    let t103917 = 12.0_f64 * t12345 * t29427 * t1555;
    let t103925 = 4.0_f64 * t97991 * t5900;
    (t103900, t103905, t103909, t103914, t103917, t103925)
}
