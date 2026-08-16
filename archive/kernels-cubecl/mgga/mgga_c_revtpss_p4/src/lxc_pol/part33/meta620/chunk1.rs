//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2059/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2059<F: Float>(t5775: F, t689: F, t7242: F, t25898: F, t98040: F, t25901: F, t25878: F, t98356: F, t27989: F, t94921: F, t94802: F, t25899: F, t98303: F) -> (F, F, F, F, F, F, F) {
    let t98379 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7242 * t5775;
    let t98380 = t98040 * t25898;
    let t98382 = F::cast_from(0.25702851531048074406e-1_f64) * t98380 * t25901;
    let t98384 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t98356;
    let t98387 = F::cast_from(0.14456046980341999104e-1_f64) * t94921 * t27989;
    let t98390 = F::cast_from(0.25702851531048074406e-1_f64) * t94802 * t27989;
    let t98399 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t98303;
    (t98379, t98380, t98382, t98384, t98387, t98390, t98399)
}
