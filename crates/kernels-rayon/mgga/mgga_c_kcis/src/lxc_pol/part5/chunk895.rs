//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 895/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk895(t7515: f64, t7532: f64, t2128: f64, t7276: f64, t7278: f64, t7280: f64, t7284: f64, t7288: f64, t7290: f64, t7292: f64, t7294: f64, t7297: f64, t7300: f64, t7302: f64, t7306: f64) -> (f64, f64, f64) {
    let t7533 = t7515 + t7532;
    let t7537 = t2128 * t2128;
    let t7552 = -0.44965277777777777777e-2_f64 * t7276 - 0.5e0_f64 * t7278 + 0.125e0_f64 * t7280 - 0.9375e-1_f64 * t7284 - 0.13489583333333333333e-1_f64 * t7288 + 0.10791666666666666667e0_f64 * t7290 - 0.26979166666666666666e-1_f64 * t7292 + 0.20234375e-1_f64 * t7294 - 0.10791666666666666667e0_f64 * t7297 + 0.26979166666666666666e-1_f64 * t7300 - 0.1875e0_f64 * t7302 + 0.101171875e-1_f64 * t7306;
    (t7533, t7537, t7552)
}
