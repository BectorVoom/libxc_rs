//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1416/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1416<F: Float>(t12560: F, t12561: F, t12562: F, t12563: F, t12564: F, t12565: F, t9225: F, t3951: F, t604: F, t1406: F, t2239: F, t1437: F, t2241: F) -> (F, F, F, F) {
    let t12566 = t12560 - t12561 + t12562 - t12563 + t12564 + t12565 - t9225;
    let t12568 = t3951 * t604;
    let t12571 = t1406 * t2239;
    let t12582 = t1437 * t2241;
    (t12566, t12568, t12571, t12582)
}
