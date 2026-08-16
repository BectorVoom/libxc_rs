//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 652/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk652(t1616: f64, t3808: f64, t3666: f64, t3671: f64, t3676: f64, t3681: f64, t3685: f64, t3689: f64, t3710: f64, t3692: f64, t3704: f64, t3715: f64, t3719: f64) -> (f64, f64) {
    let t3810 = 2.0_f64 * t1616 * t3808;
    let t3811 = 0.40483072916666666669e-4_f64 * t3666;
    let t3812 = 0.34752370105806885418e-3_f64 * t3671;
    let t3813 = 0.25301920572916666668e-5_f64 * t3676;
    let t3814 = 0.21720231316129303386e-4_f64 * t3681;
    let t3815 = 0.2318836277704281739e-4_f64 * t3685;
    let t3816 = 0.67530371184977617164e-6_f64 * t3689;
    let t3819 = 0.21103240995305505364e-7_f64 * t3710;
    let t3822 = t3811 - t3812 - t3813 + t3814 - t3815 - t3816 + 0.57970906942607043474e-5_f64 * t3692 - 0.49166375783284505217e-8_f64 * t3704 + t3819 + 0.66297786877786731988e-7_f64 * t3715 - 0.90579542097823505425e-7_f64 * t3719;
    (t3810, t3822)
}
