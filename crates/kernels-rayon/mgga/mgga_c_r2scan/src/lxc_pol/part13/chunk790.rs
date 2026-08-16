//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 790/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk790(t4695: f64, t4881: f64, t4883: f64, t4886: f64, t4892: f64, t4894: f64, t4896: f64, t4898: f64, t4703: f64, t4880: f64, t4891: f64, t4901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6943 = 2.0_f64 * t4695;
    let t6946 = 12.0_f64 * t4881;
    let t6947 = 40.0_f64 * t4883;
    let t6948 = 80.0_f64 * t4886;
    let t6949 = 4.0_f64 * t4892;
    let t6950 = 4.0_f64 * t4894;
    let t6951 = 32.0_f64 * t4896;
    let t6952 = 24.0_f64 * t4898;
    let t6953 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952 - t4901;
    (t6943, t6946, t6947, t6948, t6949, t6950, t6951, t6952, t6953)
}
