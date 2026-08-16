//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 699/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk699(t190: f64, t4864: f64, t8286: f64, t147: f64, t19: f64, t457: f64, t3156: f64, t1458: f64, t442: f64, t567: f64, t3116: f64, t2937: f64, t4026: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8287 = t4864 * t190;
    let t8288 = t8286 * t8287;
    let t8290 = t457 * t19 * t147;
    let t8291 = t3156 * t8290;
    let t8292 = t8288 * t8291;
    let t8294 = t1458 * t190;
    let t8295 = t8286 * t8294;
    let t8296 = t442 * t567;
    let t8297 = t3116 * t8296;
    let t8298 = t8295 * t8297;
    let t8300 = t2937 * t4026;
    (t8290, t8291, t8292, t8296, t8297, t8298, t8300)
}
