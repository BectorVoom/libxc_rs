//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 844/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk844(t285: f64, t3013: f64, t545: f64, t39: f64, t991: f64, t159: f64, t2522: f64, t532: f64, t510: f64, t967: f64, t5651: f64, t1083: f64, t1473: f64) -> (f64, f64, f64, f64, f64) {
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    let t8287 = t532 * t2522;
    let t8290 = 0.58113483035773838734e-3_f64 * t8287 * t159 * t285;
    let t8292 = t967 * t510;
    let t8293 = t5651 * t8292;
    let t8296 = t1473 * t1083;
    (t8277, t8281, t8290, t8293, t8296)
}
