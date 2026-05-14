//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 613/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk613<F: Float>(t1616: F, t3808: F, t3666: F, t3671: F, t3676: F, t3681: F, t3685: F, t3689: F, t3710: F, t3725: F, t3730: F, t3735: F, t3740: F, t338: F, t1096: F, t3565: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3810 = 2.0 * t1616 * t3808;
    let t3811 = 0.40483072916666666669e-4 * t3666;
    let t3812 = 0.34752370105806885418e-3 * t3671;
    let t3813 = 0.25301920572916666668e-5 * t3676;
    let t3814 = 0.21720231316129303386e-4 * t3681;
    let t3815 = 0.2318836277704281739e-4 * t3685;
    let t3816 = 0.67530371184977617164e-6 * t3689;
    let t3819 = 0.21103240995305505364e-7 * t3710;
    let t3828 = 0.16414765573575218917e-4 * t3725 - 0.23485962392041415794e-4 * t3730 - 0.34197428278281706076e-6 * t3735 + 0.14678726495025884871e-5 * t3740;
    let t3829 = t3828 * t338;
    let t3830 = t3565 * t1096;
    (t3810, t3811, t3812, t3813, t3814, t3815, t3816, t3819, t3828, t3829, t3830)
}
