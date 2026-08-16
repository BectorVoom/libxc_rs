//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 703/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk703(t1587: f64, t3149: f64, t4495: f64, t20022: f64, t8276: f64, t8275: f64, t1781: f64, t463: f64, t20039: f64, t3134: f64, t20130: f64, t8327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20345 = t1587 * t3149 * t4495;
    let t20348 = t8276 * t20022;
    let t20349 = t8275 * t20348;
    let t20352 = t1781 * t20022;
    let t20353 = t463 * t20352;
    let t20356 = t3134 * t20039;
    let t20359 = t8327 * t20130;
    (t20345, t20348, t20349, t20352, t20353, t20356, t20359)
}
