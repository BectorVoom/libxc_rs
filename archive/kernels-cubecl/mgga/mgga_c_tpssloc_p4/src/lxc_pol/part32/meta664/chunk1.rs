//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2096/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096<F: Float>(t11147: F, t491: F, t1089: F, t1751: F, t7327: F, t1653: F, t7330: F, t85822: F, t131: F, t1419: F, t23598: F, t467: F) -> (F, F, F, F) {
    let t94797 = t491 * t11147;
    let t94837 = t7327 * t1751 * t1089;
    let t94847 = t85822 * t1653 * t7330;
    let t94858 = t1419 * t23598 * t131 * t467;
    (t94797, t94837, t94847, t94858)
}
