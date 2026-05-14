//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1320/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1320<F: Float>(t105465: F, t105468: F, t105471: F, t105476: F, t105480: F, t105483: F, t105487: F, t105491: F, t105495: F, t105499: F, t95100: F, t96062: F, t2185: F, t23652: F, t23657: F, t27147: F) -> (F, F) {
    let t105500 = -2.0 / 3.0 * t105465 + t105468 + 4.0 / 3.0 * t105471 + 2.0 / 3.0 * t105476 - 12.0 * t105480 - t105483 + t96062 - t105487 + 4.0 / 27.0 * t95100 - 6.0 * t105491 + 4.0 / 9.0 * t105495 - t105499;
    let t105505 = t23657 * t2185 * t23652 * t27147;
    (t105500, t105505)
}
