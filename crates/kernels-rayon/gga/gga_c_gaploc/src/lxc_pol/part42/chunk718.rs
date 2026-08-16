//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 718/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk718(t14313: f64, t14326: f64, t14338: f64, t14346: f64, t502: f64, t1052: f64, t3749: f64, t3009: f64, t3720: f64, t1445: f64, t12256: f64, t13045: f64, t13591: f64, t13595: f64, t13597: f64, t13600: f64, t13604: f64, t13606: f64, t13608: f64, t13611: f64, t13849: f64, t13852: f64, t2087: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14348 = t14313 + t14326 + t14338 + t14346;
    let t14349 = t502 * t14348;
    let t14350 = t1052 * t3749;
    let t14357 = t3009 * t3720;
    let t14358 = t1445 * t14357;
    let t14361 = t13591 - t13595 + t13597 + t13600 - t13604 + 0.76685851907841499354e0_f64 * t13849 - 0.76685851907841499354e0_f64 * t13852 - 0.21450293971110256002e1_f64 * t12256 * t13045 - 0.13803453343411469884e2_f64 * t2087 * t14358 - t13606 - t13608 + t13611;
    (t14348, t14349, t14350, t14357, t14358, t14361)
}
