//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3168/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3168<F: Float>(t459: F, t83211: F, t83230: F, t1230: F, t17396: F, t17736: F, t20265: F, t20797: F, t20959: F, t20963: F, t21300: F, t225: F, t24680: F, t3626: F, t4181: F, t480: F, t484: F, t5230: F, t5348: F, t5354: F, t57005: F, t57710: F, t6425: F, t70140: F, t70800: F, t71036: F, t71039: F, t71081: F) -> (F, F) {
    let t83232 = (t83211 + t83230) * t459;
    let t83240 = F::cast_from(0.68598428988911579154e-2_f64) * t71081 * t5348 + F::cast_from(0.34299214494455789577e-2_f64) * t17396 * t21300 - F::cast_from(0.64311027177104605458e-3_f64) * t70800 * t5354 - F::cast_from(0.20579528696673473746e-1_f64) * t71036 * t20959 + F::cast_from(0.20579528696673473746e-1_f64) * t71039 * t20963 - F::cast_from(0.34299214494455789577e-2_f64) * t57710 * t20797 - F::cast_from(0.25724410870841842183e-2_f64) * t57005 * t3626 * t20265 * t4181 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t6425 * t5230 + F::cast_from(0.85748036236139473944e-3_f64) * t70140 + F::cast_from(0.21437009059034868486e-3_f64) * t83232 * t225 * t480 * t484 - F::cast_from(0.53100265402527852012e-1_f64) * t1230 * t24680 * t484;
    (t83232, t83240)
}
