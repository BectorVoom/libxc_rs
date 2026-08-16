//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 914/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk914(t17188: f64, t587: f64, t1897: f64, t4991: f64, t1634: f64, t5463: f64, t639: f64, t155: f64, t1639: f64, t1644: f64, t1648: f64, t5288: f64) -> (f64, f64, f64, f64, f64) {
    let t17189 = t587 * t17188;
    let t17190 = 128.0_f64 / 1215.0_f64 * t17189;
    let t17192 = t587 * t4991 * t1897;
    let t17193 = 32.0_f64 / 135.0_f64 * t17192;
    let t17195 = t639 * t5463 * t1634;
    let t17196 = 16.0_f64 / 135.0_f64 * t17195;
    let t17197 = t155 * t1639;
    let t17199 = t639 * t17197 * t1644;
    let t17200 = 16.0_f64 / 81.0_f64 * t17199;
    let t17202 = 16.0_f64 / 45.0_f64 * t1648 * t5288;
    (t17190, t17193, t17196, t17200, t17202)
}
