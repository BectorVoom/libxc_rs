//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3168/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3168(t459: f64, t83211: f64, t83230: f64, t1230: f64, t17396: f64, t17736: f64, t20265: f64, t20797: f64, t20959: f64, t20963: f64, t21300: f64, t225: f64, t24680: f64, t3626: f64, t4181: f64, t480: f64, t484: f64, t5230: f64, t5348: f64, t5354: f64, t57005: f64, t57710: f64, t6425: f64, t70140: f64, t70800: f64, t71036: f64, t71039: f64, t71081: f64) -> (f64, f64) {
    let t83232 = (t83211 + t83230) * t459;
    let t83240 = 0.68598428988911579154e-2_f64 * t71081 * t5348 + 0.34299214494455789577e-2_f64 * t17396 * t21300 - 0.64311027177104605458e-3_f64 * t70800 * t5354 - 0.20579528696673473746e-1_f64 * t71036 * t20959 + 0.20579528696673473746e-1_f64 * t71039 * t20963 - 0.34299214494455789577e-2_f64 * t57710 * t20797 - 0.25724410870841842183e-2_f64 * t57005 * t3626 * t20265 * t4181 - 0.17149607247227894789e-2_f64 * t17736 * t3626 * t6425 * t5230 + 0.85748036236139473944e-3_f64 * t70140 + 0.21437009059034868486e-3_f64 * t83232 * t225 * t480 * t484 - 0.53100265402527852012e-1_f64 * t1230 * t24680 * t484;
    (t83232, t83240)
}
