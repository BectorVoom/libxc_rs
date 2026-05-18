//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1043/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1043<F: Float>(t3772: F, t816: F, t13173: F, t2133: F, t1076: F, t13368: F, t5: F) -> (F, F, F, F) {
    let t44220 = t816 * t3772;
    let t44230 = t13173 * t2133;
    let t44246 = t1076 * t816;
    let t44254 = t5 * t13368;
    (t44220, t44230, t44246, t44254)
}
