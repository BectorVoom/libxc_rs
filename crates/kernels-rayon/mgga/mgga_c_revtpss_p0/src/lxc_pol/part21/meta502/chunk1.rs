//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2116/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2116(t15886: f64, t225: f64, t366: f64, t1058: f64, t4794: f64, t1651: f64, t3151: f64) -> (f64, f64, f64, f64) {
    let t15887 = t15886 * t225;
    let t15888 = t15887 * t366;
    let t15892 = 0.15244095330869239812e-2_f64 * t4794 * t1058;
    let t15893 = t1651 * t3151;
    (t15887, t15888, t15892, t15893)
}
