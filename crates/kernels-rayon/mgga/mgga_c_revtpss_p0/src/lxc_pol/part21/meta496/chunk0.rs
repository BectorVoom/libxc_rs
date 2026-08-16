//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2093/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2093(t11710: f64, t4787: f64, t3091: f64, t245: f64, t4890: f64, t3088: f64) -> (f64, f64, f64, f64) {
    let t15682 = t11710 * t4787;
    let t15684 = 0.19055119163586549765e-3_f64 * t3091 * t15682;
    let t15687 = t4890 * t245;
    let t15688 = t3088 * t15687;
    (t15682, t15684, t15687, t15688)
}
