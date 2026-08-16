//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1201/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1201(t13861: f64, t51666: f64, t13918: f64, t2249: f64, t13952: f64, t2210: f64, t14122: f64, t14125: f64, t2113: f64, t833: f64, t850: f64, t2354: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51667 = t51666 * t13861;
    let t51678 = t2249 * t13918;
    let t51682 = t13952 * t2210;
    let t51683 = t51682 * t14122;
    let t51688 = t850 * t2113 * t14125 * t833;
    let t51714 = t859 * t2354;
    (t51667, t51678, t51682, t51683, t51688, t51714)
}
