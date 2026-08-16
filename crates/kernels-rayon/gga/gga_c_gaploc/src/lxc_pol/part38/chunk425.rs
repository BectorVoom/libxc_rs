//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 425/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk425(t103: f64, t8: f64, t417: f64, t62: f64, t1234: f64, t89: f64, t1238: f64, t142: f64, t1246: f64, t458: f64, t462: f64, t153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3816 = t8 * t103;
    let t3831 = t62 * t417;
    let t4061 = t1234 * t89;
    let t4072 = t1238 * t1238;
    let t4074 = 1.0_f64 / t4072 / t142;
    let t4077 = pi * t1246 * t458;
    let t4080 = t462 * t462;
    let t4081 = 1.0_f64 / t4080;
    let t4082 = t153 * t4081;
    (t3816, t3831, t4061, t4072, t4074, t4077, t4080, t4081, t4082)
}
