//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 891/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk891(t16972: f64, t219: f64, t5400: f64, t649: f64, t5399: f64, t9: f64, t17037: f64, t155: f64, t188: f64, t213: f64, t1365: f64, t670: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17322 = t219 * t16972;
    let t17331 = t5400 * t649;
    let t17440 = t9 * t5399;
    let t17444 = t219 * t17037;
    let t17470 = t155 * t188;
    let t17508 = t155 * t213;
    let t17548 = 0.22443641344164119597e0_f64 * t670 * t1365 * t671;
    (t17322, t17331, t17440, t17444, t17470, t17508, t17548)
}
