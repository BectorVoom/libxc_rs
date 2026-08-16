//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1211/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1211(t44776: f64, t71239: f64, t71277: f64, t71299: f64, t71306: f64, t71320: f64, t83722: f64, t83728: f64, t83770: f64, t83772: f64, t83781: f64, t83789: f64, t83792: f64, t90335: f64) -> f64 {
    let t91195 = 4.0_f64 / 27.0_f64 * t83722 + t71239 + 20.0_f64 / 243.0_f64 * t83728 + t71277 + t44776 + 4.0_f64 / 3.0_f64 * t90335 + 2.0_f64 / 9.0_f64 * t83770 - 4.0_f64 / 27.0_f64 * t83772 + 4.0_f64 / 9.0_f64 * t83781 - 4.0_f64 / 9.0_f64 * t83789 + 4.0_f64 / 3.0_f64 * t83792 - t71299 + t71306 - t71320;
    t91195
}
