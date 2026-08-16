//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 940/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk940(t328: f64, t6552: f64, t331: f64, t863: f64, t19: f64, t2298: f64, t56: f64, t1: f64, t16192: f64, t191: f64, t2251: f64, t2276: f64, t6383: f64) -> (f64, f64, f64, f64) {
    let t21296 = t6552 * t328;
    let t21298 = t863 * t21296 * t331;
    let t21328 = t56 * t2298 * t19;
    let t21361 = t191 * t16192 * t1;
    let t21399 = t2276 * t2251 * t6383;
    (t21298, t21328, t21361, t21399)
}
