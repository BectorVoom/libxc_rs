//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 1002/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1002(t2739: f64, t992: f64, t2875: f64, t2874: f64, t2413: f64, t4150: f64, t2881: f64, t10771: f64, t10773: f64, t10804: f64, t11593: f64, t15500: f64, t15502: f64, t15504: f64, t15508: f64, t15511: f64, t15515: f64, t15519: f64, t15524: f64, t15528: f64, t15532: f64, t1901: f64, t446: f64) -> f64 {
    let t15533 = t992 * t2739;
    let t15534 = t2875 * t15533;
    let t15535 = t2874 * t15534;
    let t15538 = t4150 * t2413;
    let t15539 = t2881 * t15538;
    let t15543 = t10771 / 9.0_f64 - 8.0_f64 / 27.0_f64 * t10773 - t15500 - t15502 - t446 * t15504 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t15508 + 2.0_f64 / 9.0_f64 * t1901 * t15511 + 8.0_f64 / 27.0_f64 * t11593 * t15515 - 2.0_f64 / 9.0_f64 * t1901 * t15519 + t1901 * t15524 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t15528 - t15532 + t1901 * t15535 / 9.0_f64 + t1901 * t15539 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t10804;
    t15543
}
