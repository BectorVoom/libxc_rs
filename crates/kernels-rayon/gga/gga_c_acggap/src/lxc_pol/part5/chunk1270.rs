//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1270/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1270(t1165: f64, t3456: f64, t4241: f64, t5862: f64, t1180: f64, t1181: f64, t13974: f64, t13985: f64, t14001: f64, t14003: f64, t14005: f64, t14015: f64, t14017: f64, t14019: f64, t18062: f64, t18066: f64, t5207: f64, t5922: f64) -> f64 {
    let t23454 = t3456 * t1165 * t5862 * t4241;
    let t23470 = -0.85748036236139473944e-3_f64 * t23454 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t5922 * t5207 + 0.12004725073059526352e-1_f64 * t13974 - 0.85748036236139473944e-3_f64 * t13985 + 0.51448821741683684366e-2_f64 * t14001 - 0.25724410870841842183e-2_f64 * t14003 + 0.51448821741683684367e-2_f64 * t14005 - 0.34299214494455789578e-2_f64 * t18062 - 0.85748036236139473944e-3_f64 * t18066 - 0.51448821741683684367e-2_f64 * t14015 - 0.45351183609335988444e-1_f64 * t14017 + 0.68026775414003982664e-1_f64 * t14019;
    t23470
}
