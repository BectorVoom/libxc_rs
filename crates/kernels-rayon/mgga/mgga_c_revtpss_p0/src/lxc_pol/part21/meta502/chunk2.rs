//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2117/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2117(t15893: f64, t3155: f64, t3117: f64, t3162: f64, t11243: f64, t72: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15894 = t15893 * t3155;
    let t15895 = t3117 * t15894;
    let t15898 = t15893 * t3162;
    let t15899 = t3117 * t15898;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    (t15894, t15895, t15898, t15899, t15904, t15905)
}
