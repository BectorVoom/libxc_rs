//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 251/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk251(t213: f64, t218: f64, t62: f64, t689: f64, t215: f64, t220: f64, t43: f64, t126: f64, t173: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t978 = -t62 - t689;
    let t981 = piecewise3(t214, 0.0_f64, 4.0_f64 / 3.0_f64 * t215 * t978);
    let t982 = -t978;
    let t985 = piecewise3(t219, 0.0_f64, 4.0_f64 / 3.0_f64 * t220 * t982);
    let t987 = (t981 + t985) * t43;
    let t991 = t126 * t173;
    (t978, t982, t987, t991)
}
