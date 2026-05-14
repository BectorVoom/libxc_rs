//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 874/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk874<F: Float>(t1006: F, t7121: F, t1764: F, t3345: F, t1620: F, t17508: F, t3512: F, t1673: F, t3488: F, t16942: F, t3530: F, t587: F, t10325: F, t586: F, t1778: F, t3479: F) -> (F, F, F, F, F, F, F) {
    let t30666 = t1006 * t7121;
    let t30740 = t3345 * t1764;
    let t30824 = t1620 * t17508 * t3512;
    let t30839 = t3488 * t1673;
    let t30856 = t587 * t16942 * t3530;
    let t30876 = t10325 * t586;
    let t30889 = t3479 * t1778;
    (t30666, t30740, t30824, t30839, t30856, t30876, t30889)
}
