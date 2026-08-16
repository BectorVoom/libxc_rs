//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2153/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153<F: Float>(t26308: F, t5234: F, t5252: F, t6396: F, t80820: F, t19962: F, t22833: F, t19894: F, t19886: F, t5293: F, t91100: F, t19991: F) -> (F, F, F, F, F, F, F) {
    let t97217 = t5234 * t26308 * t5252;
    let t97219 = t80820 * t6396;
    let t97221 = t22833 * t19962;
    let t97223 = t22833 * t19894;
    let t97225 = t22833 * t19886;
    let t97227 = t91100 * t5293;
    let t97229 = t22833 * t19991;
    (t97217, t97219, t97221, t97223, t97225, t97227, t97229)
}
