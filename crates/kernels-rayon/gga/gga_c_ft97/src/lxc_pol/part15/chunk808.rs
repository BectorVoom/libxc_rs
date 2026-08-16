//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 808/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk808(t21373: f64, t801: f64, t10883: f64, t13538: f64, t18096: f64, t18107: f64, t18115: f64, t21353: f64, t21357: f64, t21360: f64, t21364: f64, t21367: f64, t21371: f64, t21821: f64, t4068: f64, t4977: f64) -> (f64, f64) {
    let t21825 = t801 * t21373;
    let t21837 = 0.1760655e0_f64 * t21821 - 0.352131e0_f64 * t4068 * t4977 + 0.234754e0_f64 * t21825 - t10883 - 0.19257444444444444444e0_f64 * t13538 + 0.9628722222222222222e-1_f64 * t18096 - 0.28886166666666666666e0_f64 * t18107 + 0.14443083333333333333e0_f64 * t18115 - 0.1604787037037037037e0_f64 * t21353 + 0.57772333333333333332e0_f64 * t21357 - 0.28886166666666666666e0_f64 * t21360 - 0.86658499999999999998e0_f64 * t21364 + 0.86658499999999999998e0_f64 * t21367 - 0.14443083333333333333e0_f64 * t21371;
    (t21825, t21837)
}
