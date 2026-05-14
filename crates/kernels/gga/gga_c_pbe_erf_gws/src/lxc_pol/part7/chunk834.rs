//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 834/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk834<F: Float>(t16762: F, t7115: F, t7759: F, t1416: F, t422: F, t5211: F, t617: F, t7491: F, t1407: F, t418: F, t5218: F, t562: F, t7049: F, t5217: F, t735: F, t5221: F) -> (F, F, F, F) {
    let t17128 = 16.0 / 9.0 * t7115 * t7759 * t16762;
    let t17133 = 32.0 / 9.0 * t5211 * t7491 * t1416 * t617 * t422;
    let t17138 = 32.0 / 9.0 * t5218 * t7049 * t1407 * t562 * t418;
    let t17139 = t5217 * t735;
    let t17140 = t17139 * t5221;
    (t17128, t17133, t17138, t17140)
}
