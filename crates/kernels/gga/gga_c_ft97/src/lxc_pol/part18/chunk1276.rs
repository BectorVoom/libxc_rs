//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1276/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1276<F: Float>(t165: F, t7800: F, t1359: F, t3588: F, t614: F, t6615: F, t11437: F, t12645: F, t1647: F, t1651: F, t1969: F, t23413: F, t24102: F, t24122: F, t26567: F, t26569: F, t26581: F, t26783: F, t26805: F, t26809: F, t27426: F, t3052: F, t379: F, t5772: F, t5773: F, t5845: F, t6580: F, t925: F, t94234: F, t9432: F, t94332: F) -> (F,) {
    let t104331 = t165 * t7800;
    let t104336 = t1359 * t3588;
    let t104364 = t6615 * t614;
    let t104375 = 2.0 / 9.0 * t5772 * t27426 * t104331 * t11437 - t5772 * t1969 * t104336 * t379 / 9.0 - 2.0 / 27.0 * t94234 - t23413 * t26805 / 9.0 - t5772 * t1969 * t94332 * t925 / 9.0 + t5772 * t1969 * t26783 * t1647 / 9.0 - 2.0 / 9.0 * t26809 * t1969 * t24102 * t3052 + 2.0 * t5772 * t9432 * t5773 * t12645 + t6580 * t24122 / 3.0 - t23413 * t26569 / 9.0 - t5772 * t1969 * t104364 * t379 / 9.0 - t5772 * t1969 * t26567 * t1651 / 18.0 + t26581 * t5845 / 3.0;
    (t104375,)
}
