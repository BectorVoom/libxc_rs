//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1161/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1161(t20732: f64, t2250: f64, t2182: f64, t20495: f64, t824: f64, t2271: f64, t6670: f64, t822: f64, t6674: f64, t20206: f64, t2407: f64, t858: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64) {
    let t20733 = t2250 * t20732;
    let t20734 = t2182 * param_a_c;
    let t20739 = t824 * t20495;
    let t20743 = t2271 * t6670;
    let t20744 = t822 * t20743;
    let t20746 = t20744 * t6674 / 4.0_f64;
    let t20748 = t2407 * t858 * t20206;
    (t20733, t20734, t20739, t20746, t20748)
}
