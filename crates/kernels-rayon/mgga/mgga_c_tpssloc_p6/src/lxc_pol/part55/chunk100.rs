//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 100/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk100(t253: f64, t259: f64, t144: f64, t186: f64, t189: f64, t193: f64, t202: f64) -> (f64, f64, f64) {
    let t261 = t253 * t259 + 1.0_f64;
    let t262 = f64::ln(t261);
    let t265 = t193 * t202 * t262 - t144 + t186 + t189;
    (t261, t262, t265)
}
