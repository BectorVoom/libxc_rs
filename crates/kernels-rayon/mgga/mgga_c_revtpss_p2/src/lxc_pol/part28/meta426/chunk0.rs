//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1604/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1604(t15964: f64, t3092: f64, t11659: f64, t3154: f64, t1592: f64, t357: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15965 = t3092 * t15964;
    let t15968 = t11659 * t3154;
    let t15969 = t1592 * t15968;
    let t15970 = t3092 * t15969;
    let t15973 = t11659 * t357;
    let t15974 = t1592 * t15973;
    let t15975 = t3092 * t15974;
    let t15984 = t11710 * t4782;
    let t15986 = 0.19055119163586549765e-3_f64 * t3091 * t15984;
    let t15987 = t140 * t1014;
    let t15988 = t15987 * t4579;
    (t15965, t15970, t15975, t15984, t15986, t15988)
}
