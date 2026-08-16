//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 743/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk743(t2100: f64, t817: f64, t2106: f64, t814: f64, t816: f64, t322: f64, t2108: f64, t745: f64, t1452: f64, t2102: f64, t2107: f64, t323: f64, t4867: f64, t6084: f64, t818: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6086 = t2100 * t817;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = 1.0_f64 / t6094;
    let t6096 = t322 * t6095;
    let t6097 = t2108 * t745;
    let t6100 = t745 * t1452;
    let t6104 = -3.0_f64 * t1452 * t2102 + 6.0_f64 * t2107 * t6100 + 6.0_f64 * t2108 * t6089 + t323 * t6084 - t4867 * t818 - 3.0_f64 * t6086 * t745 - 6.0_f64 * t6096 * t6097;
    (t6086, t6089, t6094, t6095, t6096, t6097, t6100, t6104)
}
