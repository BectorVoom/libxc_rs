//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 871/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk871(t1114: f64, t6566: f64, t3116: f64, t6605: f64, t343: f64, t8890: f64, t858: f64, t2407: f64, t2142: f64, t3113: f64, t1136: f64, t6228: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9119 = t1114 * t6566;
    let t9123 = 7.0_f64 / 144.0_f64 * t3116 * t6605;
    let t9125 = t8890 * t343;
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9142 = 7.0_f64 / 144.0_f64 * t3113 * t2142;
    let t9144 = t6228 * t1136;
    (t9119, t9123, t9125, t9127, t9142, t9144)
}
