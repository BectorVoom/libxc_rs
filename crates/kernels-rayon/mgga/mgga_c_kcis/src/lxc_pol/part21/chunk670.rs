//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 670/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk670(t1291: f64, t1872: f64, t5044: f64, t5049: f64, t5051: f64, t5054: f64, t5056: f64, t5058: f64, t5060: f64, t5063: f64, t5065: f64, t5069: f64, t5071: f64, t5074: f64, t5079: f64) -> (f64, f64) {
    let t5363 = t1872 * t1291;
    let t5379 = -0.9375e-1_f64 * t5044 + 0.1875e0_f64 * t5049 - 0.13489583333333333333e-1_f64 * t5051 + 0.25e0_f64 * t5054 - 0.25e0_f64 * t5056 + 0.625e-1_f64 * t5058 + 0.625e-1_f64 * t5060 - 0.625e-1_f64 * t5063 - 0.13489583333333333333e-1_f64 * t5065 + 0.101171875e-1_f64 * t5069 - 0.9375e-1_f64 * t5071 + 0.13489583333333333333e-1_f64 * t5074 - 0.20833333333333333333e-1_f64 * t5079;
    (t5363, t5379)
}
