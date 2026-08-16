//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 190/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk190(t142: f64, t782: f64, t143: f64, t684: f64, t126: f64, t60: f64, t15: f64, t130: f64, t2: f64, t4: f64, t88: f64, t128: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t783 = t142 * t782;
    let t784 = t684 * t143;
    let t787 = t60 * t126;
    let t788 = t787 * t15;
    let t789 = t130 * t2;
    let t790 = t4 * t88;
    let t791 = t789 * t790;
    let t794 = t128 * t97;
    (t783, t784, t787, t788, t789, t791, t794)
}
