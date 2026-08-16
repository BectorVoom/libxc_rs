//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1269/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1269(t12009: f64, t3150: f64, t1032: f64, t3043: f64, t1040: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64) -> (f64, f64, f64, f64, f64) {
    let t12010 = t3150 * t12009;
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = 1.0_f64 / t3145 / t334;
    (t12010, t12021, t12046, t12047, t12050)
}
