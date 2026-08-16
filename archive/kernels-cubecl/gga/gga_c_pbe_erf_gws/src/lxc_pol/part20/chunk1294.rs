//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1294/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1294<F: Float>(t2409: F, t35428: F, t3959: F, t14791: F, t353: F, t3721: F, t859: F, t11553: F, t50998: F, t53156: F, t1123: F, t51021: F, t56246: F, t810: F) -> (F, F, F, F) {
    let t56483 = t3959 * t2409 * t35428;
    let t56491 = t859 * t353 * t14791 * t3721;
    let t56495 = t50998 * t53156 * t11553;
    let t56500 = t50998 * t51021 * t1123 * t56246 * t810;
    (t56483, t56491, t56495, t56500)
}
