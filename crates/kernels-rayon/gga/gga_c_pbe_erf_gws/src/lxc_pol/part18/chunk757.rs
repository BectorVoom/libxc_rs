//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 757/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk757(t1321: f64, t83: f64, t4607: f64, t1224: f64, t1229: f64, t1232: f64, t1254: f64, t1262: f64, t1300: f64, t1305: f64, t1315: f64, t1320: f64, t1323: f64, t155: f64, t174: f64, t435: f64, t442: f64, t457: f64, t4664: f64, t4674: f64, t4679: f64, t4682: f64, t4688: f64, t4689: f64, t4697: f64, t4701: f64, t4711: f64, t4714: f64, t4718: f64, t4719: f64, t4723: f64, t4730: f64, t4735: f64) -> (f64, f64) {
    let t4737 = 1.0_f64 / t1321 / t83;
    let t4738 = t4607 * t4737;
    let t4741 = -t4664 + 1.0_f64 * t435 * t4674 + 0.20691336878655965246e4_f64 * t4679 * t4682 - t4688 + 0.32530742648344572643e-1_f64 * t174 * t4689 * t1305 + 0.10274e0_f64 * t174 * t155 * t1229 * t1232 - 0.48159446095139119799e0_f64 * t174 * t4697 * t1323 + 0.21687161765563048428e-1_f64 * t174 * t4701 * t457 - 0.16265371324172286321e-1_f64 * t174 * t1300 * t1315 - t4711 + t4714 + t4718 - 0.16522997748472177549e1_f64 * t174 * t4719 * t1262 + 0.68493333333333333332e-1_f64 * t174 * t4723 * t442 - 0.51369999999999999999e-1_f64 * t174 * t1224 * t1254 + 0.35089340384731224426e1_f64 * t1320 * t4730 + 0.1025389702100779493e4_f64 * t4735 * t4738;
    (t4737, t4741)
}
