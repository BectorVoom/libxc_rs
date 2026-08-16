//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 715/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk715(t5084: f64, t5087: f64, t5089: f64, t5092: f64, t5094: f64, t5097: f64, t5100: f64, t5166: f64, t5170: f64, t5173: f64, t5178: f64, t5183: f64, t5186: f64) -> f64 {
    let t5393 = -0.9375e-1_f64 * t5084 + 0.71944444444444444443e-1_f64 * t5087 + 0.101171875e-1_f64 * t5089 - 0.625e-1_f64 * t5092 + 0.53958333333333333333e-1_f64 * t5094 - 0.53958333333333333333e-1_f64 * t5097 + 0.13489583333333333333e-1_f64 * t5100 + 0.9375e-1_f64 * t5166 - 0.101171875e-1_f64 * t5170 + 0.101171875e-1_f64 * t5173 - 0.20234375e-1_f64 * t5178 - 0.44965277777777777777e-2_f64 * t5183 - 0.16666666666666666667e0_f64 * t5186;
    t5393
}
