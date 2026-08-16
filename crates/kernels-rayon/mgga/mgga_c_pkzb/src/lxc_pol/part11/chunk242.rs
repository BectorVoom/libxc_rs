//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 242/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk242(t790: f64, t799: f64, t307: f64, t311: f64, t786: f64) -> (f64, f64) {
    let t800 = t790 * t799;
    let t803 = 0.65854491829355115987e0_f64 * t786 * t311 - 0.65854491829355115987e0_f64 * t307 * t800;
    (t800, t803)
}
