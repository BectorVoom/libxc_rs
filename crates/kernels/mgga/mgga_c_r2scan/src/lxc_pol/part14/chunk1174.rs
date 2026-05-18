//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1174/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1174<F: Float>(t32094: F, t792: F, t1561: F, t3274: F, t97: F, t32212: F, t1234: F, t2867: F, t11582: F, t38248: F, t38249: F, t5086: F) -> (F, F, F, F, F, F) {
    let t40566 = t32094 * t792;
    let t40574 = t97 * t3274 * t1561;
    let t40575 = t32212 * t792;
    let t40579 = t2867 * t1234;
    let t40587 = t38248 * t11582 * t38249;
    let t40594 = t97 * t3274 * t5086;
    (t40566, t40574, t40575, t40579, t40587, t40594)
}
