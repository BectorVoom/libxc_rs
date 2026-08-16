//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3086/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086(t247: f64, t42792: f64, t4757: f64, t4837: f64, t15850: f64, t3111: f64, t3091: f64, t43240: f64, t4782: f64, t41296: f64, t42471: f64, t11977: f64, t4820: f64) -> (f64, f64, f64, f64, f64) {
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53433 = t15850 * t3111;
    let t53437 = t3091 * t43240 * t4782;
    let t53473 = t42471 * t41296;
    let t53479 = t11977 * t4820;
    (t53431, t53433, t53437, t53473, t53479)
}
