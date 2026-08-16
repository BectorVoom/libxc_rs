//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 775/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk775(t21369: f64, t683: f64, t92: f64, t13538: f64, t18096: f64, t18107: f64, t18115: f64, t21353: f64, t21357: f64, t21360: f64, t21364: f64, t21367: f64, t9557: f64) -> (f64, f64, f64) {
    let t21370 = t683 * t21369;
    let t21371 = t92 * t21370;
    let t21373 = -t9557 - 4.0_f64 / 9.0_f64 * t13538 + 2.0_f64 / 9.0_f64 * t18096 - 2.0_f64 / 3.0_f64 * t18107 + t18115 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t21353 + 4.0_f64 / 3.0_f64 * t21357 - 2.0_f64 / 3.0_f64 * t21360 - 2.0_f64 * t21364 + 2.0_f64 * t21367 - t21371 / 3.0_f64;
    (t21370, t21371, t21373)
}
