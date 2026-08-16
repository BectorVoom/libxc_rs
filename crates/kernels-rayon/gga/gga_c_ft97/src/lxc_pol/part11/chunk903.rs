//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 903/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk903(t37391: f64, t464: f64, t1775: f64, t8263: f64, t8278: f64, t11755: f64, t11756: f64, t11761: f64, t1588: f64, t1755: f64, t1800: f64, t2: f64, t24: f64, t37415: f64, t37430: f64, t38254: f64, t38504: f64, t38506: f64, t38508: f64, t38513: f64, t38519: f64, t38525: f64, t38526: f64, t432: f64, t462: f64, t463: f64, t469: f64, t7750: f64, t7815: f64, t92: f64) -> (f64, f64) {
    let t38534 = t464 * t37391;
    let t38538 = t1775 * t8263;
    let t38545 = t1775 * t8278;
    let t38547 = 112.0_f64 / 27.0_f64 * t38504 + 8.0_f64 * t38506 + 24.0_f64 * t92 * t24 * t38508 * t37430 - 8.0_f64 / 3.0_f64 * t38513 + 6.0_f64 * t92 * t24 * t1800 * t37415 + 16.0_f64 / 3.0_f64 * t38519 - t92 * t24 * t469 * t38254 + t38525 + 8.0_f64 / 3.0_f64 * t11755 * t11756 * t38526 - 8.0_f64 * t11761 * t1800 * t432 * t7815 - t462 * t463 * t38534 / 3.0_f64 - 8.0_f64 * t38538 - 36.0_f64 * t462 * t7750 * t2 * t1588 * t1755 + 40.0_f64 / 81.0_f64 * t38545;
    (t38534, t38547)
}
