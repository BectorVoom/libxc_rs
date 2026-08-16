//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1106/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1106(t15191: f64, t1058: f64, t4794: f64, t11243: f64, t72: f64, t3088: f64, t12078: f64, t1086: f64, t4746: f64, t3090: f64, t1065: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15876 = 0.55555555555555555556e-2_f64 * t15191;
    let t15892 = 0.15244095330869239812e-2_f64 * t4794 * t1058;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    let t15906 = t12078 * t15905;
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    let t15935 = t1065 * t2852;
    (t15876, t15892, t15904, t15905, t15906, t15926, t15935)
}
