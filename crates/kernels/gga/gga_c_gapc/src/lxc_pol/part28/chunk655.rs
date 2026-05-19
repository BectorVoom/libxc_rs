//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 655/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk655<F: Float>(t1616: F, t3808: F, t3666: F, t3671: F, t3676: F, t3681: F, t3685: F, t3689: F, t3710: F, t3692: F, t3704: F, t3715: F, t3719: F) -> (F, F) {
    let t3810 = F::new(2.0) * t1616 * t3808;
    let t3811 = F::cast_from(0.40483072916666666669e-4_f64) * t3666;
    let t3812 = F::cast_from(0.34752370105806885418e-3_f64) * t3671;
    let t3813 = F::cast_from(0.25301920572916666668e-5_f64) * t3676;
    let t3814 = F::cast_from(0.21720231316129303386e-4_f64) * t3681;
    let t3815 = F::cast_from(0.2318836277704281739e-4_f64) * t3685;
    let t3816 = F::cast_from(0.67530371184977617164e-6_f64) * t3689;
    let t3819 = F::cast_from(0.21103240995305505364e-7_f64) * t3710;
    let t3822 = t3811 - t3812 - t3813 + t3814 - t3815 - t3816 + F::cast_from(0.57970906942607043474e-5_f64) * t3692 - F::cast_from(0.49166375783284505217e-8_f64) * t3704 + t3819 + F::cast_from(0.66297786877786731988e-7_f64) * t3715 - F::cast_from(0.90579542097823505425e-7_f64) * t3719;
    (t3810, t3822)
}
