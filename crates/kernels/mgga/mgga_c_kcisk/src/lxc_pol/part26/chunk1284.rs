//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1284/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1284<F: Float>(t21499: F, t33376: F, t32026: F, t33451: F, t32066: F, t1333: F, t33558: F, t1413: F, t5867: F, t33527: F, t9442: F, t33532: F, t33482: F, t33561: F, t33550: F, t32105: F, t9792: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114674 = t33376 * t21499;
    let t114712 = 0.26805555555555555556e-2 * t32026 * t33451;
    let t114714 = 0.26805555555555555556e-2 * t32066 * t33451;
    let t114715 = t1333 * t33558;
    let t114716 = 0.33163888888888888888e-2 * t114715;
    let t114720 = t5867 * t1413;
    let t114728 = 0.69444444444444444446e-2 * t33527 * t9442;
    let t114738 = t1333 * t33532;
    let t114773 = 0.18518518518518518519e-1 * t33482 * t9442;
    let t114774 = t1333 * t33561;
    let t114783 = 0.18518518518518518519e-1 * t33550 * t9442;
    let t114784 = t9792 * t32105;
    (t114674, t114712, t114714, t114715, t114716, t114720, t114728, t114738, t114773, t114774, t114783, t114784)
}
