//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 922/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk922(t1645: f64, t2586: f64, t3307: f64, t9420: f64, t813: f64, t3280: f64, t549: f64, t2033: f64, t325: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9972 = t1645 * t2586;
    let t9981 = t9420 * t3307;
    let t9982 = t813 * t9981;
    let t10004 = t549 * t3280;
    let t10006 = 0.59584149919750711116e-1_f64 * t2033 * t10004;
    let t10007 = t40 * t325;
    (t9972, t9981, t9982, t10004, t10006, t10007)
}
