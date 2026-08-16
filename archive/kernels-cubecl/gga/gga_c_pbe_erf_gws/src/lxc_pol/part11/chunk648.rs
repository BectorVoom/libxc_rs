//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 648/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk648<F: Float>(t2132: F, t2306: F, t1477: F, t863: F, t864: F, t2263: F, t328: F, t331: F, t2157: F, t343: F) -> (F, F, F, F, F) {
    let t6216 = t2306 * t2132;
    let t6228 = t863 * t864 * t1477;
    let t6238 = t2263 * t328;
    let t6240 = t863 * t6238 * t331;
    let t6241 = t2157 * t343;
    (t6216, t6228, t6238, t6240, t6241)
}
