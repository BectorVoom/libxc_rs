//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3103/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3103(t15752: f64, t16049: f64, t16087: f64, t53884: f64, t15988: f64, t3241: f64, t1011: f64, t15158: f64, t15987: f64, t15994: f64, t43537: f64, t53668: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54261 = t16049 * t15752;
    let t54289 = t16087 * t53884;
    let t54303 = t3241 * t15988;
    let t54306 = t1011 * t15987 * t15158;
    let t54314 = t3241 * t15994;
    let t54316 = t43537 * t53668;
    (t54261, t54289, t54303, t54306, t54314, t54316)
}
