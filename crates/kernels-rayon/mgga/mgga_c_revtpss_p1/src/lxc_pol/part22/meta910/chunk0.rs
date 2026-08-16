//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3113/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113(t15731: f64, t3169: f64, t15816: f64, t3168: f64, t11710: f64, t15591: f64, t3091: f64, t16060: f64, t3241: f64, t1011: f64, t140: f64, t16122: f64) -> (f64, f64, f64, f64, f64) {
    let t54733 = t3169 * t15731;
    let t54739 = t15816 * t3168;
    let t54785 = t3091 * t11710 * t15591;
    let t54792 = t3241 * t16060;
    let t54795 = t1011 * t140 * t16122;
    (t54733, t54739, t54785, t54792, t54795)
}
