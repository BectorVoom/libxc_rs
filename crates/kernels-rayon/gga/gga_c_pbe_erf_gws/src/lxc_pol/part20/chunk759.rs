//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 759/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk759(t728: f64, t4607: f64, t4734: f64, t4737: f64, t470: f64, t1396: f64, t1399: f64, t449: f64, t456: f64, t4619: f64, t1392: f64, t1218: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4767 = 1.0_f64 / t728;
    let t4782 = t4734 * t4607 * t4737;
    let t4783 = t470 * t4782;
    let t4784 = 0.1025389702100779493e4_f64 * t4783;
    let t4785 = t1399 * t1396;
    let t4788 = t449 * t4619 * t456;
    let t4789 = t470 * t4788;
    let t4790 = 0.58482233974552040708e0_f64 * t4789;
    let t4791 = t1399 * t1392;
    let t4796 = t1399 * t1218;
    (t4767, t4784, t4785, t4790, t4791, t4796)
}
