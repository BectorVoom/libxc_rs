//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1294/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1294(t3409: f64, t5986: f64, t1163: f64, t1181: f64, t1894: f64, t4210: f64, t1173: f64, t13286: f64, t13287: f64, t14339: f64, t18588: f64, t18605: f64, t18607: f64, t18611: f64, t18616: f64, t18620: f64, t18622: f64, t23445: f64, t23718: f64, t525: f64, t530: f64) -> f64 {
    let t24026 = t3409 * t5986;
    let t24042 = t1163 * t1181 * t1894 * t4210;
    let t24048 = 0.24009450146119052704e-1_f64 * t24026 + 0.17149607247227894789e-1_f64 * t18588 - 0.13719685797782315831e-1_f64 * t13286 * t13287 * t525 * t23718 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t530 * t23445 + 0.68598428988911579156e-2_f64 * t18605 + 0.32012600194825403606e-1_f64 * t18607 + 0.17149607247227894789e-2_f64 * t18611 + 0.85748036236139473944e-3_f64 * t24042 + 0.34299214494455789578e-2_f64 * t18616 - 0.17149607247227894789e-2_f64 * t18620 - 0.16006300097412701803e-1_f64 * t18622 - 0.32012600194825403606e-1_f64 * t14339;
    t24048
}
