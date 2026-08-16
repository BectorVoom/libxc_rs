//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 964/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk964(t11655: f64, t11705: f64, t3742: f64, t883: f64, t3746: f64, t972: f64, t1096: f64, t3449: f64, t3795: f64, t3765: f64, t7553: f64, t3679: f64, t7557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11706 = t11655 + t11705;
    let t11708 = t3742 * t883;
    let t11718 = t3746 * t972;
    let t11721 = t1096 * t3449;
    let t11725 = t3795 * t972;
    let t11728 = t7553 * t3765;
    let t11730 = t3679 * t7557;
    (t11706, t11708, t11718, t11721, t11725, t11728, t11730)
}
