//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1000/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1000<F: Float>(t12627: F, t7527: F, t12818: F, t2612: F, t48067: F, t48069: F, t48071: F, t48076: F, t48078: F, t48080: F, t48082: F, t48084: F, t48086: F, t24785: F, t12782: F, t5211: F, t7106: F) -> (F, F, F, F, F) {
    let t48088 = 32.0 / 9.0 * t7527 * t12627;
    let t48090 = 16.0 / 9.0 * t2612 * t12818;
    let t48091 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 - t48082 - t48084 + t48086 + t48088 - t48090;
    let t48092 = 128.0 / 1215.0 * t24785;
    let t48095 = 32.0 / 15.0 * t5211 * t7106 * t12782;
    (t48088, t48090, t48091, t48092, t48095)
}
