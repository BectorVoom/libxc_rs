//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1173/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1173(t14873: f64, t14899: f64, t14940: f64, t14967: f64, t14985: f64, t15000: f64, t15016: f64, t15094: f64, t2053: f64, t4233: f64, t944: f64, t1167: f64, t14364: f64) -> (f64, f64, f64, f64) {
    let t15097 = t14873 + t14899 + t14940 + t14967 + t14985 + t15000 + t15016 + t15094;
    let t15101 = t4233 * t2053;
    let t15102 = t15101 * t944;
    let t15108 = t14364 * t1167;
    (t15097, t15101, t15102, t15108)
}
