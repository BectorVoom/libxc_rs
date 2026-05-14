//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1273/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1273<F: Float>(t107997: F, t107999: F, t108001: F, t108003: F, t108061: F, t122682: F, t122686: F, t122689: F, t122692: F, t122697: F, t122701: F, t122708: F, t123819: F, t108070: F, t108073: F, t122706: F, t122712: F, t122716: F, t122720: F, t123817: F, t123823: F, t123827: F, t123830: F) -> (F, F) {
    let t124495 = t107997 + t107999 - t108001 + t108003 - t122682 / 18.0 + t122686 - t122689 / 3.0 - t122692 / 3.0 + t108061 - t122697 / 2.0 - 3.0 * t122701;
    let t124497 = t122708 / 9.0;
    let t124502 = 2.0 / 9.0 * t123819;
    let t124507 = -3.0 / 4.0 * t122706 - t124497 + 12.0 * t122712 - 6.0 * t122716 - 12.0 * t122720 - t123817 / 2.0 - t124502 + 2.0 / 3.0 * t123823 - 2.0 / 3.0 * t123827 + 4.0 / 3.0 * t123830 + 4.0 / 9.0 * t108070 - t108073;
    (t124495, t124507)
}
