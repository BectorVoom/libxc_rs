//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2052/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2052(t5775: f64, t689: f64, t7242: f64, t25898: f64, t98040: f64, t25901: f64, t25878: f64, t98356: f64, t27989: f64, t94921: f64, t94802: f64, t25899: f64, t98303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98379 = 0.10975748638225852664e-1_f64 * t689 * t7242 * t5775;
    let t98380 = t98040 * t25898;
    let t98382 = 0.25702851531048074406e-1_f64 * t98380 * t25901;
    let t98384 = 0.51405703062096148812e-1_f64 * t25878 * t98356;
    let t98387 = 0.14456046980341999104e-1_f64 * t94921 * t27989;
    let t98390 = 0.25702851531048074406e-1_f64 * t94802 * t27989;
    let t98399 = 0.25702851531048074406e-1_f64 * t25899 * t98303;
    (t98379, t98380, t98382, t98384, t98387, t98390, t98399)
}
