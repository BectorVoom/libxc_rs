//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1153/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1153<F: Float>(t2189: F, t343: F, t858: F, t866: F, t867: F, t6553: F, t899: F, t922: F, t6557: F, t2113: F, t6491: F, t850: F, t860: F) -> (F, F, F, F, F) {
    let t20618 = t2189 * t2189;
    let t20619 = t20618 * t343;
    let t20623 = t866 * t867 * t858 * t20619 / F::new(32.0);
    let t20625 = t899 * t6553 * t922;
    let t20626 = t20625 * t6557;
    let t20631 = t850 * t2113 * t6491 * t860 / F::new(32.0);
    (t20618, t20619, t20623, t20626, t20631)
}
