//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk901(t1641: f64, t1696: f64, t16973: f64, t11: f64, t5089: f64, t17003: f64, t17007: f64, t17011: f64, t17016: f64, t17020: f64, t17024: f64, t17026: f64, t17028: f64, t17030: f64, t17032: f64, t17034: f64) -> (f64, f64, f64, f64) {
    let t17037 = 1.0_f64 / t1641 / t1696;
    let t17038 = t17037 * t16973;
    let t17040 = t11 * t5089 * t17038;
    let t17042 = -0.45340000000000000002e-1_f64 * t17003 + 0.37783333333333333335e-2_f64 * t17007 + 0.5037777777777777778e-2_f64 * t17011 + 0.12594444444444444445e-1_f64 * t17016 - 0.4534e-1_f64 * t17020 + 0.6801e-1_f64 * t17024 - 0.10075555555555555556e-1_f64 * t17026 - 0.15113333333333333333e-1_f64 * t17028 + 0.15113333333333333333e-1_f64 * t17030 - 0.5037777777777777778e-2_f64 * t17032 + 0.10075555555555555556e-1_f64 * t17034 - 0.2518888888888888889e-1_f64 * t17040;
    (t17037, t17038, t17040, t17042)
}
