//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 717/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk717<F: Float>(t2135: F, t2170: F, t6220: F, t2168: F, t2319: F, t2339: F, t1477: F, t863: F, t864: F, t877: F, t2156: F, t874: F) -> (F, F, F, F, F, F) {
    let t6222 = t2170 * t2135 * t6220;
    let t6224 = t2168 * t6222 / 16.0;
    let t6225 = t2319 * t2339;
    let t6228 = t863 * t864 * t1477;
    let t6229 = t6228 * t877;
    let t6230 = 35.0 / 144.0 * t6229;
    let t6231 = t2156 * t874;
    (t6222, t6224, t6225, t6228, t6230, t6231)
}
