//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 884/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk884<F: Float>(t3088: F, t419: F, t85491: F, t37749: F, t420: F, t85469: F, t37389: F, t7742: F) -> (F, F, F) {
    let t85493 = t419 * t3088 * t85491;
    let t85498 = t419 * t420 * t37749 * t85469;
    let t85501 = 24.0 * t7742 + 24.0 * t37389;
    (t85493, t85498, t85501)
}
