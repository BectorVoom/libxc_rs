//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1021/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1021(t33231: f64, t7458: f64, t1873: f64, t5449: f64, t2040: f64, t127553: f64, t22574: f64, t24432: f64, t1442: f64, t33553: f64, t5457: f64, t8595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128516 = 4.0_f64 * t7458 * t33231;
    let t128521 = t5449 * t1873;
    let t128523 = 2.0_f64 * t128521 * t2040;
    let t128535 = 6.0_f64 * t22574 * t24432 * t127553;
    let t128537 = 2.0_f64 * t1442 * t33553;
    let t128539 = 2.0_f64 * t5457 * t8595;
    (t128516, t128521, t128523, t128535, t128537, t128539)
}
