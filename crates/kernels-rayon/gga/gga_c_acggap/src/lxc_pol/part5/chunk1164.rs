//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1164/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1164(t1008: f64, t5574: f64, t1165: f64, t1180: f64, t1181: f64, t1531: f64, t1532: f64, t15982: f64, t20961: f64, t20963: f64, t20969: f64, t20972: f64, t20985: f64, t20987: f64, t20995: f64, t20999: f64, t21001: f64, t406: f64, t4267: f64, t4417: f64, t4463: f64, t5752: f64, t929: f64) -> f64 {
    let t21003 = t1008 * t5574;
    let t21005 = -0.68598428988911579156e-2_f64 * t20961 + 0.34299214494455789578e-2_f64 * t1531 * t1181 * t1532 * t20963 * t406 - 0.32012600194825403606e-1_f64 * t20969 - 0.51448821741683684368e-2_f64 * t1180 * t1165 * t4417 * t20972 + 0.17149607247227894789e-2_f64 * t1531 * t1181 * t1532 * t5752 * t929 - 0.25724410870841842183e-2_f64 * t15982 - 0.34299214494455789578e-2_f64 * t20985 - 0.68598428988911579156e-1_f64 * t4463 * t1181 * t4267 * t20987 - 0.13719685797782315831e-1_f64 * t20995 + 0.42874018118069736972e-3_f64 * t20999 + 0.4801890029223810541e-1_f64 * t21001 - 0.10289764348336736874e-1_f64 * t21003;
    t21005
}
