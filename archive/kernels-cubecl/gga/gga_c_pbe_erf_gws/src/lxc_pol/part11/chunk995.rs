//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 995/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk995<F: Float>(t3749: F, t6717: F, t20378: F, t3912: F, t11777: F, t6183: F, t20940: F, t3837: F, t1114: F, t3747: F, t6670: F, t3871: F, t6505: F) -> (F, F, F, F, F, F) {
    let t36920 = t6717 * t3749;
    let t36962 = t3912 * t20378;
    let t37138 = t6183 * t11777;
    let t37257 = t20940 * t3837;
    let t37286 = t1114 * t3747 * t6670;
    let t37363 = t6505 * t3871;
    (t36920, t36962, t37138, t37257, t37286, t37363)
}
