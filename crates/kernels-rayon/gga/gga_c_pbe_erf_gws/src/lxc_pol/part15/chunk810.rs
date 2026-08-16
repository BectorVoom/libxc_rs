//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 810/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk810(t337: f64, t6409: f64, t2121: f64, t2115: f64, t2142: f64, t2276: f64, t6401: f64, t2281: f64, t2100: f64, t369: f64, t814: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6410 = t337 * t6409;
    let t6411 = t2121 * t6410;
    let t6414 = t2115 * t2142;
    let t6416 = t2276 * t6401;
    let t6417 = t6416 * t2281;
    let t6421 = t2100 * t369;
    let t6424 = t814 * t931;
    (t6411, t6414, t6416, t6417, t6421, t6424)
}
