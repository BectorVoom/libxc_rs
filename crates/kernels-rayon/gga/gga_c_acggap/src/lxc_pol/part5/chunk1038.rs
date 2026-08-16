//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1038/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1038(t17550: f64, t4469: f64, t13092: f64, t4269: f64, t3431: f64, t4701: f64, t13850: f64, t3360: f64, t1165: f64, t3491: f64, t530: f64, t1507: f64, t3573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17870 = t17550 * t4469;
    let t17876 = t13092 * t4269;
    let t17886 = t3431 * t4701;
    let t17888 = t3360 * t13850;
    let t17891 = t17888 * t1165 * t530 * t3491;
    let t17895 = t3573 * t1507;
    (t17870, t17876, t17886, t17888, t17891, t17895)
}
