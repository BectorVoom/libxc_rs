//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1117/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1117(t14967: f64, t14969: f64, t14972: f64, t14984: f64, t14986: f64, t14999: f64, t15003: f64, t15005: f64, t11829: f64, t15008: f64, t15010: f64, t11775: f64, t11778: f64, t11780: f64, t11792: f64, t11825: f64, t11828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19995 = 0.70178683471615754484e1_f64 * t14967;
    let t19996 = 48.0_f64 * t14969;
    let t19997 = 96.0_f64 * t14972;
    let t19998 = 0.11696447245269292414e1_f64 * t14984;
    let t19999 = 64.0_f64 * t14986;
    let t20000 = 0.11696447245269292414e1_f64 * t14999;
    let t20001 = 0.23392894490538584828e1_f64 * t15003;
    let t20002 = 0.20508037716432813315e4_f64 * t15005;
    let t20003 = 8.0_f64 * t11829;
    let t20004 = 8.0_f64 * t15008;
    let t20005 = 8.0_f64 * t15010;
    let t20006 = -t11775 + t11778 - t11780 + t11792 + t11825 - t19995 - t19996 + t19997 - t19998 - t19999 + t11828 - t20000 - t20001 - t20002 - t20003 - t20004 - t20005;
    (t19995, t19996, t19997, t19998, t19999, t20000, t20001, t20002, t20003, t20004, t20005, t20006)
}
