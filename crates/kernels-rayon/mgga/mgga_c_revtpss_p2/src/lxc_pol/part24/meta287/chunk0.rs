//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1067/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1067(t19826: f64, t3161: f64, t1058: f64, t6318: f64, t1062: f64, t15670: f64, t247: f64, t3109: f64, t6096: f64, t1063: f64, t140: f64, t6284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19827 = t3161 * t19826;
    let t19867 = t6318 * t1058;
    let t19878 = t15670 * t1062;
    let t19882 = t247 * t3109 * t6096;
    let t19883 = t1063 * t19882;
    let t19900 = t140 * t6284;
    (t19827, t19867, t19878, t19882, t19883, t19900)
}
