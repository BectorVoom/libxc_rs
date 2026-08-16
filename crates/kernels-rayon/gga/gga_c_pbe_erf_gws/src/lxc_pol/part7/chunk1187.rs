//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1187/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1187(t274: f64, t322: f64, t6094: f64, t2108: f64, t1452: f64, t18987: f64, t20992: f64, t2102: f64, t2107: f64, t21074: f64, t21077: f64, t21082: f64, t323: f64, t4867: f64, t6086: f64, t6089: f64, t6096: f64, t6097: f64, t6100: f64, t745: f64, t818: f64) -> f64 {
    let t21091 = t322 / t6094 / t274;
    let t21092 = t2108 * t2108;
    let t21098 = t1452 * t1452;
    let t21105 = -36.0_f64 * t1452 * t2108 * t6096 + 8.0_f64 * t2107 * t4867 * t745 - 6.0_f64 * t1452 * t6086 - t18987 * t818 + t20992 * t323 - 4.0_f64 * t2102 * t4867 + 6.0_f64 * t2107 * t21098 - 4.0_f64 * t21074 * t745 + 12.0_f64 * t21077 * t2108 - 24.0_f64 * t21082 * t6097 + 24.0_f64 * t21091 * t21092 + 24.0_f64 * t6089 * t6100;
    t21105
}
