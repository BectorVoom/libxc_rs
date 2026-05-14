//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 988/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk988<F: Float>(t7433: F, t8779: F, t4979: F, t7561: F, t4983: F, t7822: F, t1181: F, t21955: F, t30806: F, t599: F, t4987: F, t7647: F, t4364: F, t4963: F, t2937: F, t524: F, t943: F) -> (F, F, F, F, F, F, F, F) {
    let t35307 = t7433 * t8779;
    let t35308 = 0.25724410870841842184e-2 * t35307;
    let t35309 = t7561 * t4979;
    let t35311 = t7822 * t4983;
    let t35315 = t30806 * t1181 * t599 * t21955;
    let t35316 = 0.64311027177104605458e-2 * t35315;
    let t35317 = t7647 * t4987;
    let t35318 = 0.17149607247227894789e-2 * t35317;
    let t35319 = t7822 * t4364;
    let t35321 = t7561 * t4963;
    let t35324 = t524 * t2937 * t943;
    (t35308, t35309, t35311, t35316, t35318, t35319, t35321, t35324)
}
