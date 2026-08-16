//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3401/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401(t1100: f64, t5019: f64, t18898: f64, t41813: f64, t981: f64, t19023: f64, t3022: f64, t41520: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64) -> (f64, f64, f64, f64) {
    let t63827 = t1100 * t5019;
    let t63833 = 0.10254018858216406658e4_f64 * t981 * t18898 * t41813;
    let t63835 = 0.11696447245269292414e1_f64 * t3022 * t19023;
    let t63847 = 0.37083333333333333334e-1_f64 * t63274 - 0.12361111111111111111e-1_f64 * t63276 + 0.41203703703703703704e-2_f64 * t63278 - 0.12361111111111111111e-1_f64 * t63281 - 0.61805555555555555555e-2_f64 * t63285 - 0.10300925925925925926e-1_f64 * t63290 + 0.37083333333333333334e-1_f64 * t63293 + 0.18541666666666666667e-1_f64 * t63299 + 0.12361111111111111111e0_f64 * t63304 - 0.22249999999999999999e0_f64 * t63308 + t41520 + 0.61805555555555555556e-2_f64 * t51967;
    (t63827, t63833, t63835, t63847)
}
