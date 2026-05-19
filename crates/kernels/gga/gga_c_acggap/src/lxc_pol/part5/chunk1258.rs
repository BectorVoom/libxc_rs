//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1258/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1258<F: Float>(t1008: F, t5961: F, t5966: F, t5821: F, t1089: F, t1096: F, t1165: F, t1198: F, t13881: F, t16871: F, t17742: F, t17773: F, t17778: F, t1859: F, t1889: F, t20545: F, t3266: F, t3396: F, t418: F, t422: F, t429: F, t4818: F, t530: F, t5959: F) -> F {
    let t23179 = t1008 * t5961;
    let t23181 = t1008 * t5966;
    let t23192 = t1008 * t5821;
    let t23206 = F::cast_from(0.34299214494455789578e-2_f64) * t17742 - F::cast_from(0.64025200389650807212e-1_f64) * t17773 + F::cast_from(0.25724410870841842183e-2_f64) * t17778 - F::cast_from(0.68598428988911579156e-2_f64) * t23179 - F::cast_from(0.68598428988911579156e-2_f64) * t23181 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1089 * t1198 * t1859 - F::cast_from(0.68598428988911579156e-2_f64) * t418 * t1089 * t429 * t5959 + F::cast_from(0.42874018118069736972e-3_f64) * t13881 - F::cast_from(0.34299214494455789578e-2_f64) * t23192 - F::cast_from(0.10289764348336736873e0_f64) * t16871 * t1165 * t530 * t4818 + F::cast_from(0.41159057393346947494e-1_f64) * t3396 * t1165 * t20545 * t1096 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t422 * t3266 * t1889;
    t23206
}
