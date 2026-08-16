//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1100/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1100(t23405: f64, t35022: f64, t1349: f64, t34962: f64, t376: f64, t34799: f64, t104205: f64, t1058: f64, t138415: f64, t138533: f64, t138538: f64, t1389: f64, t1969: f64, t26535: f64, t26551: f64, t26581: f64, t26769: f64, t28: f64, t32870: f64, t32967: f64, t35012: f64, t5766: f64, t5772: f64, t5778: f64, t7309: f64, t7342: f64, t925: f64) -> f64 {
    let t147160 = t23405 * t35022;
    let t147184 = t1349 * t376 * t34962;
    let t147191 = t1349 * t376 * t34799;
    let t147195 = -t147160 / 27.0_f64 - t138533 / 18.0_f64 - t7309 * t26535 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t104205 + t1349 * t28 * t26769 * t1389 / 3.0_f64 + t1349 * t28 * t32967 * t26551 + t138538 / 54.0_f64 + t26581 * t7342 / 6.0_f64 + t1349 * t28 * t32870 * t1058 / 6.0_f64 - t147184 / 18.0_f64 - t5772 * t1969 * t138415 * t925 / 9.0_f64 - t147191 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t5766 * t35012;
    t147195
}
