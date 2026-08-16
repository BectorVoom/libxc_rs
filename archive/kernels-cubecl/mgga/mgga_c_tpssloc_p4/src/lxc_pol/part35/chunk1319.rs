//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1319/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1319<F: Float>(t28334: F, t6547: F, t28322: F, t6579: F, t1484: F, t1519: F, t23110: F, t23185: F, t28422: F, t23168: F, t28346: F, t28338: F, t81591: F) -> (F, F, F, F, F, F) {
    let t98374 = t6547 * t28334;
    let t98380 = t6579 * t28322;
    let t98389 = t1519 * t1484;
    let t98399 = t23185 * t23110 * t28422;
    let t98416 = t23168 * t28346;
    let t98420 = t81591 * t28338;
    (t98374, t98380, t98389, t98399, t98416, t98420)
}
