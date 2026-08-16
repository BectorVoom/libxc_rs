//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 789/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk789(t6006: f64, t6008: f64, t6013: f64, t6017: f64, t6021: f64, t6023: f64, t6025: f64, t6030: f64, t6032: f64, t6035: f64, t6039: f64, t6042: f64, t6045: f64) -> f64 {
    let t6255 = -0.25e0_f64 * t6006 - 0.13489583333333333333e-1_f64 * t6008 - 0.20234375e-1_f64 * t6013 - 0.9375e-1_f64 * t6017 - 0.101171875e-1_f64 * t6021 + 0.625e-1_f64 * t6023 + 0.53958333333333333333e-1_f64 * t6025 + 0.1875e0_f64 * t6030 + 0.625e-1_f64 * t6032 - 0.53958333333333333333e-1_f64 * t6035 - 0.9375e-1_f64 * t6039 - 0.16666666666666666667e0_f64 * t6042 + 0.25e0_f64 * t6045;
    t6255
}
