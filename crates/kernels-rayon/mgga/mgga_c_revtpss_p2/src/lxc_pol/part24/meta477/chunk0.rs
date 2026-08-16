//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1462/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462(t1065: f64, t372: f64, t6299: f64, t3115: f64, t42793: f64, t6272: f64, t19675: f64, t1025: f64, t371: f64, t6276: f64, t676: f64, t15749: f64, t4858: f64) -> (f64, f64, f64, f64, f64) {
    let t66777 = t372 * t1065 * t6299;
    let t67015 = t3115 * t42793 * t6272;
    let t67052 = t372 * t19675;
    let t67186 = t1025 * t371 * t676 * t6276;
    let t67195 = t4858 * t15749;
    (t66777, t67015, t67052, t67186, t67195)
}
