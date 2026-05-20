//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1186/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1186<F: Float>(t14079: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t14067: F, t213: F, t4071: F, t561: F, t5728: F, t9666: F, t9668: F, t9672: F, t9677: F, t9683: F, t9687: F, t9691: F, t9694: F) -> F {
    let t14081 = F::cast_from(0.19514881078765566038e-1_f64) * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = F::cast_from(0.19514881078765566038e-1_f64) * t14082 * t1364;
    let t14085 = t1357 * t5775;
    let t14087 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14085;
    let t14088 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t14067 * t561 - t9666 + F::cast_from(0.54878743191129263322e-2_f64) * t9668 - F::cast_from(0.9757440539382783019e-2_f64) * t9672 - F::cast_from(0.23131639038696784278e-2_f64) * t9677 + F::cast_from(0.19514881078765566038e-1_f64) * t9683 + F::cast_from(0.2601984143835408805e-1_f64) * t9687 + F::cast_from(0.26341796731742046394e1_f64) * t4071 * t5728 - t14081 + t14084 + t14087 - t9691 + t9694;
    t14088
}
