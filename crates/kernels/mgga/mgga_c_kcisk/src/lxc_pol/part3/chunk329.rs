//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 329/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk329<F: Float>(t1636: F, t1659: F, t26: F, t1638: F, t1649: F, t1651: F, t1654: F, t1658: F, t586: F) -> (F, F, F, F) {
    let t1660 = t1659 * t1636;
    let t1661 = t26 * t1660;
    let t1663 = 0.1898925e1 * t1649 - t1651 - 0.29896666666666666667e0 * t1638 + 0.3071625e0 * t1654 - t1658 - 0.82156666666666666667e-1 * t1661;
    let t1664 = 1.0 / t586;
    (t1660, t1661, t1663, t1664)
}
