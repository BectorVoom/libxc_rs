//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1322/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1322<F: Float>(t15964: F, t3092: F, t11659: F, t3154: F, t1592: F, t357: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F) -> (F, F, F, F, F) {
    let t15965 = t3092 * t15964;
    let t15968 = t11659 * t3154;
    let t15969 = t1592 * t15968;
    let t15970 = t3092 * t15969;
    let t15973 = t11659 * t357;
    let t15974 = t1592 * t15973;
    let t15975 = t3092 * t15974;
    let t15984 = t11710 * t4782;
    let t15986 = F::cast_from(0.19055119163586549765e-3_f64) * t3091 * t15984;
    let t15987 = t140 * t1014;
    let t15988 = t15987 * t4579;
    (t15965, t15970, t15975, t15986, t15988)
}
