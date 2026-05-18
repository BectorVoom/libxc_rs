//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 692/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk692<F: Float>(t19: F, t931: F, t329: F, t332: F, t838: F, t857: F, t1146: F, t2242: F, t353: F, t858: F, t1120: F, t4442: F) -> (F, F, F, F, F, F, F) {
    let t9239 = t931 * t19;
    let t9241 = t329 * t332 * t9239;
    let t9246 = t838 * t857;
    let t9270 = t329 * t9246;
    let t9275 = t2242 * t1146;
    let t9283 = t858 * t353;
    let t9290 = t4442 * t1120;
    (t9239, t9241, t9246, t9270, t9275, t9283, t9290)
}
