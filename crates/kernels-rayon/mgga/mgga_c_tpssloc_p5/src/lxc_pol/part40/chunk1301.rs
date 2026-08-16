//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1301/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1301(t110899: f64, t110904: f64, t110910: f64, t111601: f64, t111602: f64, t111604: f64, t111641: f64, t111683: f64, t1396: f64, t1398: f64, t1858: f64, t20149: f64, t2193: f64, t30218: f64, t30500: f64, t5364: f64, t5381: f64, t6471: f64, t8171: f64, t8241: f64, t8256: f64) -> f64 {
    let t111692 = 2.0_f64 * t5364 * t8256 + t1396 * t30500 + t111601 + t110899 + 2.0_f64 * t111602 + t111604 + t110904 + t1398 * (t111641 + t111683) + 2.0_f64 * t30218 * t1858 + t6471 * t8171 + t20149 * t2193 + 2.0_f64 * t8241 * t5381 + t110910;
    t111692
}
