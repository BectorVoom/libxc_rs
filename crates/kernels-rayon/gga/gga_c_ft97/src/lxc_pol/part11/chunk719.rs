//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 719/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk719(t2373: f64, t684: f64, t9770: f64, t446: f64, t2409: f64, t713: f64, t2354: f64, t9735: f64, t9739: f64, t9742: f64, t9747: f64, t9752: f64, t9755: f64, t9759: f64, t9763: f64, t9765: f64, t9768: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9771 = t684 * t2373;
    let t9772 = t9770 * t9771;
    let t9773 = t446 * t9772;
    let t9775 = t2409 * t713;
    let t9776 = t2354 * t9775;
    let t9777 = t446 * t9776;
    let t9779 = -2.0_f64 / 27.0_f64 * t9735 - t9739 / 3.0_f64 + t9742 / 3.0_f64 + t9747 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t9752 - t9755 / 9.0_f64 + t9759 / 6.0_f64 + t9763 / 6.0_f64 - t9765 / 9.0_f64 - t9768 / 9.0_f64 - t9773 / 3.0_f64 - t9777 / 3.0_f64;
    (t9771, t9772, t9773, t9775, t9776, t9777, t9779)
}
