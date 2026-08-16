//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1269/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1269(t1365: f64, t1799: f64, t1307: f64, t1347: f64, t5187: f64, t1345: f64, t1348: f64, t1819: f64, t1821: f64, t5272: f64, t5278: f64, t546: f64, t548: f64) -> (f64, f64, f64, f64) {
    let t5279 = t1365 * t1799;
    let t5280 = t5279 * t1307;
    let t5283 = t1347 * t5187;
    let t5286 = 3.0_f64 * t1345 * t1821 + 3.0_f64 * t1348 * t1819 - t5272 * t548 - 12.0_f64 * t5278 * t5280 + 3.0_f64 * t5283 * t546;
    (t5279, t5280, t5283, t5286)
}
