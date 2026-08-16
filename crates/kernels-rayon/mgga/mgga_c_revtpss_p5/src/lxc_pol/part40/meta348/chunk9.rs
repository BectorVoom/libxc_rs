//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1186/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1186(t14079: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t14067: f64, t213: f64, t4071: f64, t561: f64, t5728: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64, t9691: f64, t9694: f64) -> f64 {
    let t14081 = 0.19514881078765566038e-1_f64 * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = 0.19514881078765566038e-1_f64 * t14082 * t1364;
    let t14085 = t1357 * t5775;
    let t14087 = 0.10975748638225852664e-1_f64 * t689 * t14085;
    let t14088 = 0.65854491829355115987e0_f64 * t213 * t14067 * t561 - t9666 + 0.54878743191129263322e-2_f64 * t9668 - 0.9757440539382783019e-2_f64 * t9672 - 0.23131639038696784278e-2_f64 * t9677 + 0.19514881078765566038e-1_f64 * t9683 + 0.2601984143835408805e-1_f64 * t9687 + 0.26341796731742046394e1_f64 * t4071 * t5728 - t14081 + t14084 + t14087 - t9691 + t9694;
    t14088
}
