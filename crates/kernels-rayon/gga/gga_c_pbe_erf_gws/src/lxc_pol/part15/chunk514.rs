//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 514/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk514(t2135: f64, t2170: f64, t2171: f64, t2168: f64, t369: f64, t814: f64, t322: f64, t931: f64, t810: f64) -> (f64, f64, f64, f64, f64) {
    let t2173 = t2170 * t2135 * t2171;
    let t2175 = t2168 * t2173 / 24.0_f64;
    let t2178 = t814 * t369;
    let t2181 = t322 * t931;
    let t2182 = t810 * t810;
    (t2173, t2175, t2178, t2181, t2182)
}
