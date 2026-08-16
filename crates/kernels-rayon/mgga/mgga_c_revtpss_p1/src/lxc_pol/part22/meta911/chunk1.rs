//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3116/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3116(t11671: f64, t15925: f64, t15752: f64, t15917: f64, t127: f64, t15700: f64, t15702: f64, t4801: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64) -> (f64, f64, f64, f64) {
    let t54916 = t15925 * t11671;
    let t54919 = t15917 * t15752;
    let t54925 = t15700 * t127 * t4801 * t15702;
    let t54943 = t1063 * t247 * t11986 * t4583;
    (t54916, t54919, t54925, t54943)
}
