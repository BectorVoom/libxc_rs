//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1152/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1152(t15898: f64, t3067: f64, t1015: f64, t5243: f64, t3068: f64, t5249: f64, t1562: f64, t4056: f64, t1114: f64, t5064: f64, t9702: f64, t461: f64, t5242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15899 = t3067 * t15898;
    let t15901 = t5243 * t1015;
    let t15902 = t3068 * t15901;
    let t15905 = t5249 * t1015;
    let t15906 = t3068 * t15905;
    let t15909 = t1562 * t4056;
    let t15910 = t3068 * t15909;
    let t15913 = t5064 * t1114;
    let t15914 = t9702 * t15913;
    let t15917 = t461 * t5242;
    (t15899, t15902, t15906, t15910, t15914, t15917)
}
