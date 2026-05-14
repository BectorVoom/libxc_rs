//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1239/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1239<F: Float>(t108070: F, t110064: F, t122706: F, t122708: F, t122712: F, t122716: F, t122720: F, t123817: F, t123819: F, t123823: F, t123827: F, t123830: F, t24543: F, t30963: F, t24437: F, t24438: F, t4635: F, t6135: F, t747: F) -> (F, F, F) {
    let t123833 = -t122706 / 4.0 - t122708 / 27.0 + 4.0 * t122712 - 2.0 * t122716 - 4.0 * t122720 - t123817 / 6.0 - 2.0 / 27.0 * t123819 + 2.0 / 9.0 * t123823 - 2.0 / 9.0 * t123827 + 4.0 / 9.0 * t123830 + 4.0 / 27.0 * t108070 - t110064;
    let t123835 = t24543 * t30963;
    let t123840 = t24437 * t24438 * t6135 * t4635 * t747;
    (t123833, t123835, t123840)
}
