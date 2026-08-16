//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1293/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1293(t13092: f64, t6328: f64, t6361: f64, t997: f64, t14056: f64, t5903: f64, t3379: f64, t6161: f64, t17039: f64, t6339: f64, t1083: f64, t1165: f64, t1173: f64, t16059: f64, t175: f64, t18578: f64, t18580: f64, t18582: f64, t18584: f64, t1889: f64, t21689: f64, t372: f64, t398: f64, t418: f64, t5784: f64, t955: f64) -> f64 {
    let t24001 = t13092 * t6328;
    let t24003 = t997 * t6361;
    let t24009 = t14056 * t5903;
    let t24011 = t3379 * t6161;
    let t24013 = t17039 * t6339;
    let t24024 = 0.85748036236139473944e-3_f64 * t1173 * t1165 * t1889 * t955 - 0.64025200389650807212e-1_f64 * t24001 + 0.24009450146119052704e0_f64 * t24003 + 0.18007087609589289528e0_f64 * t418 * t16059 * t175 * t21689 + 0.13719685797782315831e-1_f64 * t24009 + 0.17149607247227894789e-2_f64 * t24011 + 0.10289764348336736873e-1_f64 * t24013 - 0.17149607247227894789e-2_f64 * t418 * t398 * t1083 * t5784 * t372 - 0.64025200389650807212e-1_f64 * t18578 - 0.32012600194825403606e-1_f64 * t18580 + 0.64025200389650807212e-1_f64 * t18582 + 0.32012600194825403606e-1_f64 * t18584;
    t24024
}
