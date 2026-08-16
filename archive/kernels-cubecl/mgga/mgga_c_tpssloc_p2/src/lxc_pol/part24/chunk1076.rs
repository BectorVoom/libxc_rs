//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1076/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1076<F: Float>(t12168: F, t1343: F, t820: F, t3799: F, t3858: F, t12267: F, t1340: F, t120: F, t3850: F, t3805: F, t3807: F, t3719: F, t550: F) -> (F, F, F, F, F, F) {
    let t12392 = t1343 * t820 * t12168;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    let t12402 = t120 * t3850;
    let t12404 = t3805 * t12402 * t3807;
    let t12407 = t550 * t3719;
    (t12392, t12395, t12397, t12402, t12404, t12407)
}
