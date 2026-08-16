//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1329/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1329(t14729: f64, t14731: f64, t14732: f64, t5414: f64, t5417: f64, t5419: f64, t5422: f64, t6022: f64, t6025: f64, t6604: f64, t6607: f64, t6612: f64, t6616: f64) -> f64 {
    let t24695 = 12.0_f64 * t5414 - 2.0_f64 * t5417 + 6.0_f64 * t6604 + 6.0_f64 * t5419 + 12.0_f64 * t6607 + 6.0_f64 * t5422 - t14729 + t14731 - t14732 + 12.0_f64 * t6612 - 0.11696447245269292414e1_f64 * t6022 - 2.0_f64 * t6616 - 0.36622894612013090108e-3_f64 * t6025;
    t24695
}
