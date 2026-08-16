//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 745/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk745(t11054: f64, t8291: f64, t10979: f64, t1787: f64, t10983: f64, t10988: f64, t8327: f64, t11665: f64, t11668: f64, t11669: f64, t11672: f64, t11676: f64, t11684: f64, t11686: f64, t11687: f64, t11691: f64, t11694: f64, t11697: f64, t11700: f64, t3051: f64, t3139: f64, t462: f64, t8283: f64, t8285: f64, t8287: f64, t8333: f64, t92: f64) -> f64 {
    let t11703 = t8291 * t11054;
    let t11706 = t1787 * t10979;
    let t11709 = t1787 * t10983;
    let t11712 = t8327 * t10988;
    let t11715 = -t92 * t11665 + t11668 - 4.0_f64 / 9.0_f64 * t11669 - 2.0_f64 / 3.0_f64 * t3051 * t11672 + 2.0_f64 * t462 * t11676 - 8.0_f64 / 27.0_f64 * t8283 + t8285 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t8287 - 2.0_f64 / 9.0_f64 * t8333 - t11684 + t11686 - 2.0_f64 / 9.0_f64 * t462 * t11687 - 10.0_f64 / 27.0_f64 * t462 * t11691 - 8.0_f64 / 9.0_f64 * t3139 * t11694 + t462 * t11697 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t462 * t11700 - 2.0_f64 / 3.0_f64 * t462 * t11703 + 2.0_f64 / 3.0_f64 * t462 * t11706 + t462 * t11709 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t462 * t11712;
    t11715
}
