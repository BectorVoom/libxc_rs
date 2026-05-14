//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1076/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1076<F: Float>(t1683: F, t9134: F, t2609: F, t6874: F, t9140: F, t22564: F, t22567: F, t22570: F, t22573: F, t22578: F, t22581: F, t22586: F, t22589: F, t22594: F, t22695: F, t22698: F) -> (F, F, F, F) {
    let t24668 = t9134 * t1683;
    let t24671 = t2609 * t6874;
    let t24674 = t9140 * t1683;
    let t24697 = -0.57386111111111111112e0 * t22567 + 0.20659e1 * t22570 + 0.13772666666666666667e1 * t22573 - 0.309885e1 * t22578 - 0.41318e1 * t22581 - 0.34431666666666666667e0 * t22586 + 0.103295e1 * t22589 - 0.104195e0 * t22695 - 0.516475e0 * t22594 + 0.23154444444444444445e-1 * t22698 + 0.11477222222222222222e0 * t22564;
    (t24668, t24671, t24674, t24697)
}
