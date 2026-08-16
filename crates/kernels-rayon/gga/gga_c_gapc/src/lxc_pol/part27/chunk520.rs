//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 520/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk520(t3004: f64, t3008: f64, t190: f64, t671: f64, t1649: f64, t1643: f64, t191: f64, t632: f64, t1045: f64, t198: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3009 = t3004 * t3008;
    let t3011 = t190 * t671;
    let t3012 = t3011 * pi;
    let t3013 = t3012 * t1649;
    let t3014 = t1643 * t3013;
    let t3016 = t632 * t191;
    let t3017 = t1045 * t198;
    (t3009, t3012, t3013, t3014, t3016, t3017)
}
