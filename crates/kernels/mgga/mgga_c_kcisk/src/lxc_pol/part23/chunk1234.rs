//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1234/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1234<F: Float>(t9515: F, t9859: F, t14612: F, t539: F, t1596: F, t2331: F, t6204: F) -> (F, F, F, F) {
    let t33778 = t9515 * t9859;
    let t33781 = t539 * t14612;
    let t33782 = t2331 * t1596;
    let t33783 = t33781 * t33782;
    let t33784 = t6204 * t33783;
    (t33778, t33781, t33783, t33784)
}
