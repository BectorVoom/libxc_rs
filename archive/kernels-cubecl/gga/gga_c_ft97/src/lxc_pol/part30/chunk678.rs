//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 678/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk678<F: Float>(t28821: F, t852: F, t193: F, t6308: F, t4129: F, t6222: F, t89: F, t1212: F, t24964: F, t2680: F, t7021: F, t824: F) -> (F, F, F, F, F, F, F) {
    let t28822 = t852 * t28821;
    let t28824 = t6308 * t193 * t28822;
    let t28827 = t6222 * t4129;
    let t28828 = t193 * t28827;
    let t28829 = t89 * t28828;
    let t28831 = t24964 * t1212;
    let t28832 = t193 * t28831;
    let t28833 = t89 * t28832;
    let t28835 = t2680 * t7021;
    let t28836 = t28835 * t824;
    (t28824, t28828, t28829, t28832, t28833, t28835, t28836)
}
