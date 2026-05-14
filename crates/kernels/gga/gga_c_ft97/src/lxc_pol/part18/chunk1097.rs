//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1097/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1097<F: Float>(t1598: F, t1711: F, t22547: F, t22528: F, t22572: F, t5569: F, t22626: F, t64: F, t22511: F, t22817: F, t3076: F, t22514: F, t2258: F) -> (F, F, F, F, F, F, F) {
    let t93014 = t1598 * t1711;
    let t93015 = t22547 * t93014;
    let t93026 = t5569 * t22572 * t22528;
    let t93034 = t64 * t1711 * t22626;
    let t93046 = t22817 * t22511;
    let t93047 = t3076 * t93046;
    let t93048 = t22514 * t2258;
    (t93014, t93015, t93026, t93034, t93046, t93047, t93048)
}
