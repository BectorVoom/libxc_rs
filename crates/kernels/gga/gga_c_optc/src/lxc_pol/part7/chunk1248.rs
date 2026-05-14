//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1248/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1248<F: Float>(t4356: F, t8905: F, t1162: F, t2367: F, t8936: F, t27481: F, t9169: F, t9171: F, t1107: F, t8914: F, t9122: F, t9124: F, t9116: F, t9118: F, t9102: F, t9104: F) -> (F, F, F, F, F, F) {
    let t27579 = t4356 * t8905;
    let t27587 = t1162 * t2367 * t8936;
    let t27590 = t9169 * t27481 * t9171;
    let t27592 = t1107 * t8914;
    let t27594 = t9122 * t27592 * t9124;
    let t27597 = t9116 * t27592 * t9118;
    let t27600 = t9102 * t27592 * t9104;
    (t27579, t27587, t27590, t27594, t27597, t27600)
}
