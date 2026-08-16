//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk862(t11514: f64, t2345: f64, t3814: f64, t13171: f64, t823: f64, t850: f64, t852: f64, t860: f64, t1076: f64, t1109: f64, t2255: f64, t3258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13531 = t2345 * t11514 * t3814;
    let t13534 = t13171 * t823;
    let t13536 = t850 * t13534 * t852;
    let t13538 = t13536 * t860 / 96.0_f64;
    let t13539 = t1076 * t1109;
    let t13541 = t2255 * t3258 * t13539;
    (t13531, t13534, t13536, t13538, t13539, t13541)
}
