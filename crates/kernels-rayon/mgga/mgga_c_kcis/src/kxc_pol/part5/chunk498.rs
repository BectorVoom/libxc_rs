//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 498/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk498(t2036: f64, t2040: f64, t2044: f64, t2048: f64, t2052: f64, t2056: f64, t2063: f64, t2067: f64) -> f64 {
    let t2128 = 0.9375e-1_f64 * t2036 - 0.9375e-1_f64 * t2040 - 0.25e0_f64 * t2044 + 0.625e-1_f64 * t2048 - 0.101171875e-1_f64 * t2052 + 0.101171875e-1_f64 * t2056 + 0.53958333333333333333e-1_f64 * t2063 - 0.13489583333333333333e-1_f64 * t2067;
    t2128
}
