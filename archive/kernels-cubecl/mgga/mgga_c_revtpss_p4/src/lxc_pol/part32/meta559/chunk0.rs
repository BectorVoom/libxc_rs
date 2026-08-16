//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1878/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1878<F: Float>(t25240: F, t3964: F, t5617: F, t786: F, t97961: F, t25898: F, t98040: F, t25081: F, t7897: F, t2: F, t2411: F, t892: F) -> (F, F, F, F, F, F) {
    let t98285 = t3964 * t25240 * t5617;
    let t98308 = t786 * t97961;
    let t98380 = t98040 * t25898;
    let t98450 = t7897 * t25081;
    let t98631 = t2411 * t2;
    let t98646 = t892 * t2;
    (t98285, t98308, t98380, t98450, t98631, t98646)
}
