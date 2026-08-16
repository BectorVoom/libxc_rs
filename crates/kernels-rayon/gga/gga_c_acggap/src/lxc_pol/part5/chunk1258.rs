//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1258/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1258(t1008: f64, t5961: f64, t5966: f64, t5821: f64, t1089: f64, t1096: f64, t1165: f64, t1198: f64, t13881: f64, t16871: f64, t17742: f64, t17773: f64, t17778: f64, t1859: f64, t1889: f64, t20545: f64, t3266: f64, t3396: f64, t418: f64, t422: f64, t429: f64, t4818: f64, t530: f64, t5959: f64) -> f64 {
    let t23179 = t1008 * t5961;
    let t23181 = t1008 * t5966;
    let t23192 = t1008 * t5821;
    let t23206 = 0.34299214494455789578e-2_f64 * t17742 - 0.64025200389650807212e-1_f64 * t17773 + 0.25724410870841842183e-2_f64 * t17778 - 0.68598428988911579156e-2_f64 * t23179 - 0.68598428988911579156e-2_f64 * t23181 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t1198 * t1859 - 0.68598428988911579156e-2_f64 * t418 * t1089 * t429 * t5959 + 0.42874018118069736972e-3_f64 * t13881 - 0.34299214494455789578e-2_f64 * t23192 - 0.10289764348336736873e0_f64 * t16871 * t1165 * t530 * t4818 + 0.41159057393346947494e-1_f64 * t3396 * t1165 * t20545 * t1096 - 0.17149607247227894789e-2_f64 * t418 * t422 * t3266 * t1889;
    t23206
}
