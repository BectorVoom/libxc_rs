//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1703/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1703(t1660: f64, t3201: f64, t1058: f64, t4798: f64, t15127: f64, t15125: f64, t15191: f64, t4794: f64, t11243: f64, t72: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15862 = t1660 * t3201;
    let t15865 = 0.28582678745379824648e-3_f64 * t4798 * t1058;
    let t15874 = 0.37037037037037037037e-2_f64 * t15127;
    let t15875 = 0.11111111111111111111e-1_f64 * t15125;
    let t15876 = 0.55555555555555555556e-2_f64 * t15191;
    let t15892 = 0.15244095330869239812e-2_f64 * t4794 * t1058;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    (t15862, t15865, t15874, t15875, t15876, t15892, t15904, t15905)
}
