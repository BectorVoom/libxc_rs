//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1116/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1116<F: Float>(t35315: F, t4987: F, t7647: F, t4364: F, t7822: F, t4963: F, t7561: F, t2937: F, t524: F, t943: F, t1165: F, t30856: F, t604: F) -> (F, F, F, F, F, F) {
    let t35316 = F::new(0.64311027177104605458e-2) * t35315;
    let t35317 = t7647 * t4987;
    let t35318 = F::new(0.17149607247227894789e-2) * t35317;
    let t35319 = t7822 * t4364;
    let t35321 = t7561 * t4963;
    let t35324 = t524 * t2937 * t943;
    let t35327 = t30856 * t1165 * t604 * t35324;
    (t35316, t35318, t35319, t35321, t35324, t35327)
}
