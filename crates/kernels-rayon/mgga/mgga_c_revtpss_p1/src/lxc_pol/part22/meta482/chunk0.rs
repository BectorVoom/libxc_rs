//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2195/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2195(t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64) -> (f64, f64, f64) {
    let t15984 = t11710 * t4782;
    let t15986 = 0.19055119163586549765e-3_f64 * t3091 * t15984;
    let t15987 = t140 * t1014;
    (t15984, t15986, t15987)
}
