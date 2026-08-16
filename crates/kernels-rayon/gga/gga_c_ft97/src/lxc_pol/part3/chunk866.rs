//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 866/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk866(t16710: f64, t16714: f64, t16717: f64, t16721: f64, t16724: f64, t16727: f64, t16730: f64, t16734: f64, t17237: f64, t17241: f64, t17244: f64, t16739: f64, t16742: f64, t16745: f64, t16748: f64, t16751: f64, t16756: f64, t16760: f64, t16922: f64, t16925: f64, t16928: f64, t17349: f64) -> (f64, f64) {
    let t17472 = 4.0_f64 / 9.0_f64 * t16710 - 2.0_f64 / 9.0_f64 * t16714 - 2.0_f64 / 3.0_f64 * t16717 + 2.0_f64 / 27.0_f64 * t16721 + 4.0_f64 / 9.0_f64 * t16724 - 10.0_f64 / 81.0_f64 * t16727 - 8.0_f64 / 27.0_f64 * t16730 + 2.0_f64 / 9.0_f64 * t16734 - t17237 / 12.0_f64 + t17241 / 8.0_f64 - t17244 / 6.0_f64;
    let t17484 = -2.0_f64 * t16739 + 4.0_f64 / 3.0_f64 * t16742 + t16745 / 27.0_f64 - 2.0_f64 / 27.0_f64 * t16748 + 2.0_f64 / 81.0_f64 * t16751 + 2.0_f64 / 3.0_f64 * t16756 - t16760 / 9.0_f64 + t17349 / 6.0_f64 - t16922 / 3.0_f64 + t16925 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t16928;
    (t17472, t17484)
}
