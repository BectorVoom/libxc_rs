//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 782/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk782<F: Float>(t2182: F, t274: F, t810: F, t824: F, t821: F, t3259: F, t814: F, t2264: F, t899: F, t923: F, t6636: F, t6684: F, t2344: F, t904: F, t4383: F, t6158: F) -> (F, F, F, F, F, F, F) {
    let t9488 = t274 * t2182;
    let t9504 = t824 * t810;
    let t9505 = t821 * t9504;
    let t9568 = t3259 * t814;
    let t9630 = t899 * t2264 * t923;
    let t9637 = t6684 * t6636;
    let t9665 = t2344 * t904;
    let t11374 = t6158 * t4383;
    (t9488, t9505, t9568, t9630, t9637, t9665, t11374)
}
