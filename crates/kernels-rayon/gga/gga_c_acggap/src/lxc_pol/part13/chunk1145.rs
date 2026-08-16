//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1145/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1145(t1347: f64, t7614: f64, t31505: f64, t31530: f64, t31532: f64, t2001: f64, t5108: f64, t1967: f64, t8502: f64, t4932: f64, t31495: f64, t31499: f64, t31501: f64, t31503: f64, t31509: f64, t31510: f64, t31514: f64, t31525: f64, t31526: f64, t31528: f64, t31543: f64) -> f64 {
    let t35709 = t7614 * t1347;
    let t35710 = 0.32012600194825403606e-1_f64 * t35709;
    let t35713 = 0.18007087609589289529e-1_f64 * t31505;
    let t35718 = 0.34299214494455789578e-2_f64 * t31530;
    let t35719 = 0.34299214494455789578e-2_f64 * t31532;
    let t35720 = t2001 * t5108;
    let t35722 = t1967 * t8502;
    let t35723 = 0.25724410870841842184e-2_f64 * t35722;
    let t35724 = t2001 * t4932;
    let t35726 = -t31495 - t31499 - t35710 + 0.32155513588552302729e-2_f64 * t31501 - 0.38586616306262763276e-2_f64 * t31503 - t35713 - t31509 - 7.0_f64 / 144.0_f64 * t31510 - 11.0_f64 / 576.0_f64 * t31514 + t31525 + 0.39624596284901231606e-1_f64 * t31526 + 0.11321313224257494744e-1_f64 * t31528 + t35718 - t35719 + 0.17149607247227894789e-1_f64 * t35720 + t31543 + t35723 + 0.68598428988911579156e-2_f64 * t35724;
    t35726
}
