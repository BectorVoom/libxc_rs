//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2198/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2198<F: Float>(t1512: F, t46667: F, t16903: F, t9638: F, t41008: F, t5568: F, t5614: F, t9674: F, t16859: F, t2639: F, t13360: F, t4257: F) -> (F, F, F, F, F, F) {
    let t58731 = t46667 * t1512;
    let t58735 = t9638 * t16903;
    let t58744 = t41008 * t5568;
    let t58759 = t9674 * t5614;
    let t58761 = t2639 * t16859;
    let t58763 = t13360 * t4257;
    (t58731, t58735, t58744, t58759, t58761, t58763)
}
