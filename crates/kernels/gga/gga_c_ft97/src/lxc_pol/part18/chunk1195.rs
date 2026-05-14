//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1195/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1195<F: Float>(t11525: F, t5674: F, t5675: F, t8411: F, t92: F, t92173: F, t1767: F, t452: F, t965: F, t1564: F, t1647: F, t25899: F, t25890: F, t93506: F, t1651: F, t1755: F, t25528: F, t28: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t101650 = t5674 * t8411 * t5675 * t11525;
    let t101651 = t92173 * t92;
    let t101655 = t101651 * t452 * t5675 * t965 * t1767;
    let t101659 = t5674 * t1564 * t25899 * t1647;
    let t101661 = t93506 * t25890;
    let t101662 = t101661 / 9.0;
    let t101665 = t5674 * t1564 * t25899 * t1651;
    let t101669 = t89 * t28 * t25528 * t1755;
    (t101650, t101655, t101659, t101661, t101662, t101665, t101669)
}
