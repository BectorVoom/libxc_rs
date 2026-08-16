//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3109/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109(t11710: f64, t15958: f64, t3091: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t3090: f64, t11262: f64, t3127: f64, t4874: f64, t15758: f64, t16055: f64) -> (f64, f64, f64, f64, f64) {
    let t54553 = t3091 * t11710 * t15958;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54599 = t3127 * t11262 * t4874;
    let t54623 = t15758 * t16055;
    (t54553, t54570, t54578, t54599, t54623)
}
