//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 695/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk695<F: Float>(t1105: F, t2264: F, t899: F, t923: F, t6636: F, t6684: F, t1158: F, t6505: F, t2344: F, t904: F, t1150: F, t6717: F) -> (F, F, F, F, F, F) {
    let t9607 = t1105 * param_a_c;
    let t9630 = t899 * t2264 * t923;
    let t9637 = t6684 * t6636;
    let t9658 = t6505 * t1158;
    let t9665 = t2344 * t904;
    let t9669 = t6717 * t1150;
    (t9607, t9630, t9637, t9658, t9665, t9669)
}
