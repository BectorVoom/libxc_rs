//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 821/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk821(t1014: f64, t1251: f64, t2781: f64, t7236: f64, t1006: f64, t1673: f64, t197: f64, t5293: f64, t1036: f64, t5463: f64, t639: f64, t188: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7407 = t1251 * t1014;
    let t7409 = t7236 * t2781;
    let t7421 = t1006 * t1673;
    let t7435 = t5293 * t197;
    let t7459 = t5463 * t1036;
    let t7460 = t639 * t7459;
    let t7467 = t331 * t188;
    (t7407, t7409, t7421, t7435, t7460, t7467)
}
