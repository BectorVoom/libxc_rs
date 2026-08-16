//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1997/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1997(t14067: f64, t14081: f64, t14084: f64, t14087: f64, t213: f64, t4071: f64, t561: f64, t5728: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64, t9691: f64, t9694: f64) -> f64 {
    let t14088 = 0.65854491829355115987e0_f64 * t213 * t14067 * t561 - t9666 + 0.54878743191129263322e-2_f64 * t9668 - 0.9757440539382783019e-2_f64 * t9672 - 0.23131639038696784278e-2_f64 * t9677 + 0.19514881078765566038e-1_f64 * t9683 + 0.2601984143835408805e-1_f64 * t9687 + 0.26341796731742046394e1_f64 * t4071 * t5728 - t14081 + t14084 + t14087 - t9691 + t9694;
    t14088
}
