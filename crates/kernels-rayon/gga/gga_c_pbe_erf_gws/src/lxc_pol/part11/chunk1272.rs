//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1272/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1272(t2155: f64, t50118: f64, t858: f64, t867: f64, t1105: f64, t1153: f64, t11994: f64, t12072: f64, t13593: f64, t20733: f64, t2255: f64, t2277: f64, t2312: f64, t274: f64, t3257: f64, t3763: f64, t37645: f64, t44283: f64, t50275: f64, t50279: f64, t50281: f64, t50286: f64, t50290: f64, t50291: f64, t50292: f64, t6637: f64, t6685: f64, t9441: f64, t9847: f64) -> (f64, f64) {
    let t50299 = t2155 * t867 * t858 * t50118 / 16.0_f64;
    let t50300 = -t2312 * t2255 * t9441 * t13593 * t1105 / 48.0_f64 + 7.0_f64 / 384.0_f64 * t2277 * t3257 * t11994 * t12072 * t274 + t50275 - t50279 + t50281 - t6637 * t37645 * t9847 * t3763 / 32.0_f64 - 5.0_f64 / 16.0_f64 * t20733 * t1153 * t50286 + t50290 + t50291 + 3.0_f64 / 64.0_f64 * t6685 * t44283 * t50292 + t50299;
    (t50299, t50300)
}
