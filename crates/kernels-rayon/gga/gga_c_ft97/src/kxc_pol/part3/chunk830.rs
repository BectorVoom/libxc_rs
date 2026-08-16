//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 830/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk830(t140: f64, t16802: f64, t16917: f64, t526: f64, t27: f64, t89: f64, t375: f64, t4715: f64, t4669: f64, t12918: f64, t16706: f64, t16710: f64, t16714: f64, t16717: f64, t16721: f64, t16724: f64, t16727: f64, t16730: f64, t16734: f64, t16739: f64, t16742: f64, t16745: f64, t16748: f64, t16751: f64, t16756: f64, t16760: f64) -> (f64, f64, f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t16919 = piecewise3(t141, t16802 + t16917, 0.0_f64);
    let t16920 = t526 * t16919;
    let t16922 = t89 * t27 * t16920;
    let t16925 = t89 * t375 * t4715;
    let t16928 = t89 * t375 * t4669;
    let t16930 = -t12918 - t16706 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t16710 - t16714 / 9.0_f64 - t16717 / 3.0_f64 + t16721 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t16724 - 5.0_f64 / 81.0_f64 * t16727 - 4.0_f64 / 27.0_f64 * t16730 + t16734 / 9.0_f64 - t16739 + 2.0_f64 / 3.0_f64 * t16742 + t16745 / 54.0_f64 - t16748 / 27.0_f64 + t16751 / 81.0_f64 + t16756 / 3.0_f64 - t16760 / 18.0_f64 - t16922 / 6.0_f64 + t16925 / 18.0_f64 - t16928 / 9.0_f64;
    (t16919, t16922, t16925, t16928, t16930)
}
