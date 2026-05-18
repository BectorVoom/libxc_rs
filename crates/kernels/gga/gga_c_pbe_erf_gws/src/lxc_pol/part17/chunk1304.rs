//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1304/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1304<F: Float>(t54119: F, t6680: F, t2134: F, t8996: F, t14015: F, t9522: F, t1150: F, t51200: F, t14028: F, t3295: F, t4023: F, t9172: F) -> (F, F, F, F, F, F) {
    let t54120 = t54119 * t6680;
    let t54122 = t2134 * t8996;
    let t54124 = t14015 * t9522;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = F::new(7.0) / F::new(576.0) * t54128;
    let t54130 = t9172 * t4023;
    (t54120, t54122, t54124, t54126, t54129, t54130)
}
