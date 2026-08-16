//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 742/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk742(t1065: f64, t999: f64, t906: f64, t1042: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t3019: f64, t3021: f64, t3024: f64, t3028: f64, t3032: f64, t3036: f64) -> (f64, f64, f64) {
    let t3128 = t1065 * t999;
    let t3129 = t3128 * t906;
    let t3130 = t1042 * t3129;
    let t3133 = -t2868 + t2871 - t2878 + t2921 + t2929 + t3019 + t3021 - t3024 + t3028 - t3032 - t3036;
    (t3129, t3130, t3133)
}
