//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 726/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk726(t20660: f64, t9016: f64, t27: f64, t89: f64, t3342: f64, t4714: f64, t28: f64, t20044: f64, t519: f64, t356: f64, t12362: f64, t16679: f64, t16745: f64, t16748: f64, t16751: f64, t16925: f64, t16928: f64, t20658: f64, t9072: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20661 = t9016 * t20660;
    let t20663 = t89 * t27 * t20661;
    let t20664 = t3342 * t4714;
    let t20666 = t89 * t28 * t20664;
    let t20667 = t519 * t20044;
    let t20669 = t89 * t356 * t20667;
    let t20676 = -2.0_f64 / 27.0_f64 * t12362 - t16679 / 9.0_f64 - t20658 / 6.0_f64 - t20663 + t20666 - t20669 / 18.0_f64 + t16745 / 18.0_f64 - t16748 / 9.0_f64 + t16751 / 27.0_f64 - t9072 + t16925 / 6.0_f64 - t16928 / 3.0_f64;
    (t20661, t20663, t20664, t20666, t20667, t20669, t20676)
}
