//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1739/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1739(t23204: f64, t6555: f64, t23164: f64, t6572: f64, t6562: f64, t212: f64, t252: f64) -> (f64, f64, f64, f64, f64) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23228 = t212 * t252;
    (t23205, t23206, t23208, t23209, t23228)
}
