//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1144/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1144(t12223: f64, t2464: f64, t2465: f64, t825: f64, t39002: f64, t787: f64, t9824: f64, t41413: f64, t41418: f64, t41422: f64, t41428: f64, t13891: f64, t2033: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47506 = t825 * t2464 * t2465 * t12223;
    let t47508 = t787 * t39002;
    let t47509 = t47508 * t9824;
    let t47511 = 0.38342925953920749677e0_f64 * t41413;
    let t47512 = 0.38342925953920749677e0_f64 * t41418;
    let t47513 = 0.85206502119823888171e-1_f64 * t41422;
    let t47515 = 0.51123901271894332903e0_f64 * t41428;
    let t47517 = t2033 * t549 * t13891;
    (t47506, t47509, t47511, t47512, t47513, t47515, t47517)
}
