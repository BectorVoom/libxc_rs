//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2220/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2220(t11137: f64, t11459: f64, t14702: f64, t14720: f64, t14946: f64, t14947: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64) -> f64 {
    let t18245 = -t11459 + 0.79148148148148148147e-2_f64 * t11137 + 0.15829629629629629629e-1_f64 * t14702 + 0.79148148148148148147e-2_f64 * t14720 - t14946 - t14947 + 0.39574074074074074073e-2_f64 * t18203 + 0.19787037037037037037e-1_f64 * t18208 - 0.71233333333333333332e-1_f64 * t18213 - 0.23744444444444444444e-1_f64 * t18217 - 0.11872222222222222222e-1_f64 * t18219 + 0.10685e0_f64 * t18223 + 0.71233333333333333332e-1_f64 * t18227 - 0.5936111111111111111e-2_f64 * t18229 - 0.11872222222222222222e-1_f64 * t18234 + 0.35616666666666666666e-1_f64 * t18239 + 0.17808333333333333333e-1_f64 * t18243;
    t18245
}
