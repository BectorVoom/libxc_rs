//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1009/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1009(t10769: f64, t828: f64, t2746: f64, t2710: f64, t2713: f64, t4371: f64, t4353: f64, t808: f64, t10744: f64, t240: f64, t849: f64, t10716: f64, t4349: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14785 = t10769 * t828;
    let t14791 = t2746 * t828;
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14832 = t849 * t240;
    let t14839 = t10716 * t4349;
    (t14785, t14791, t14817, t14819, t14820, t14832, t14839)
}
