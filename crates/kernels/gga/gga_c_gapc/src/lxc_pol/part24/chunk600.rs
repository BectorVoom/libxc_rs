//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 600/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk600<F: Float>(t3873: F, t576: F, t3725: F, t3730: F, t3735: F, t3740: F, t1125: F) -> (F, F, F) {
    let t3874 = t576 * t3873;
    let t3879 = 0.32829531147150437834e-4 * t3725 - 0.46971924784082831588e-4 * t3730 - 0.68394856556563412154e-6 * t3735 + 0.29357452990051769742e-5 * t3740;
    let t3883 = t1125 * t1125;
    (t3874, t3879, t3883)
}
