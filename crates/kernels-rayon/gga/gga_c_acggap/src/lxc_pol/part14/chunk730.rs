//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 730/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk730(t1992: f64, t407: f64, t7842: f64, t7585: f64, t2070: f64, t7839: f64, t580: f64, t7600: f64, t56: f64, t985: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7844 = t7842 * t1992 * t407;
    let t7845 = t7585 * t7844;
    let t7847 = t7839 * t2070;
    let t7849 = t7600 * t580;
    let t7850 = 77.0_f64 / 1728.0_f64 * t7849;
    let t7851 = t985 * t56;
    let t7852 = t7851 * t569;
    (t7844, t7845, t7847, t7850, t7851, t7852)
}
