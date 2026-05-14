//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 612/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk612<F: Float>(t3784: F, t3789: F, t2660: F, t3717: F, t2767: F, t3636: F, t3641: F, t3647: F, t1049: F, t3480: F, t1112: F, t2964: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3800 = 0.12147342662753799615e-3 * t3636;
    let t3801 = 0.86898242813537603825e-4 * t3641;
    let t3802 = 0.2530696388073708253e-5 * t3647;
    let t3806 = t3480 * t1049;
    let t3807 = t2964 * t1112;
    let t3808 = t1112 * t1049;
    (t3790, t3792, t3793, t3800, t3801, t3802, t3806, t3807, t3808)
}
