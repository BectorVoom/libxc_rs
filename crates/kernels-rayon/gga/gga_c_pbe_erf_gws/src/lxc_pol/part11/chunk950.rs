//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 950/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk950(t1076: f64, t153: f64, t4573: f64, t1072: f64, t168: f64, t5589: f64, t3013: f64, t700: f64, t1061: f64, t256: f64, t5426: f64, t2654: f64, t5421: f64) -> (f64, f64, f64, f64, f64) {
    let t22766 = t153 * t4573 * t1076;
    let t22778 = t168 * t5589 * t1072;
    let t22800 = t3013 * t700;
    let t22811 = t1061 * t5426 * t256;
    let t22813 = t2654 * t5421;
    (t22766, t22778, t22800, t22811, t22813)
}
