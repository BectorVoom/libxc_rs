//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1114/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1114(t187: f64, t29025: f64, t29027: f64, t29029: f64, t29030: f64, t29031: f64, t29033: f64, t29035: f64, t29038: f64, t29041: f64, t29044: f64, t29082: f64, t29092: f64, t29216: f64) -> f64 {
    let t29219 = t29025 - t29027 + t29029 - t29030 - t29031 + t29033 - t29035 - t29038 + t29041 + t29044 - t29082 + t187 * (t29092 + t29216);
    t29219
}
