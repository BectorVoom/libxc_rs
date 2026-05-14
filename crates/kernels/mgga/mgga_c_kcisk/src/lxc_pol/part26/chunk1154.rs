//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1154/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1154<F: Float>(t1415: F, t6336: F, t6318: F, t9497: F, t6313: F, t9491: F, t32255: F, t9836: F, t33674: F, t33677: F, t33679: F, t33681: F, t33683: F, t33685: F, t33687: F, t33689: F, t33691: F) -> (F, F, F, F, F) {
    let t33693 = t1415 * t6336;
    let t33695 = t9497 * t6318;
    let t33697 = t9491 * t6313;
    let t33699 = t32255 * t9836;
    let t33701 = -t33674 / 96.0 + t33677 / 24.0 + t33679 / 128.0 + t33681 / 8.0 + t33683 / 128.0 + t33685 / 24.0 - t33687 / 96.0 - t33689 / 24.0 + t33691 / 24.0 - t33693 / 24.0 - t33695 / 72.0 - t33697 / 24.0 + t33699 / 6.0;
    (t33693, t33695, t33697, t33699, t33701)
}
