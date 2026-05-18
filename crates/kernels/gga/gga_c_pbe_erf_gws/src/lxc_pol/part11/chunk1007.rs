//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1007/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1007<F: Float>(t12559: F, t1820: F, t5018: F, t12451: F, t586: F, t10843: F, t2643: F, t12543: F, t12869: F, t626: F, t1004: F, t184: F, t995: F) -> (F, F, F, F, F, F) {
    let t40327 = t1820 * t5018 * t12559;
    let t40329 = t12451 * t586;
    let t40358 = t10843 * t2643;
    let t40361 = t1820 * t5018 * t12543;
    let t40396 = t12869 * t626;
    let t40402 = t995 * t1004 * t184;
    (t40327, t40329, t40358, t40361, t40396, t40402)
}
