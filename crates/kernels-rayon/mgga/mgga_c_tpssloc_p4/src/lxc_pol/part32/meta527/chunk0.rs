//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1861/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861(t2109: f64, t26012: f64, t6509: f64, t7974: f64, t7255: f64, t7445: f64, t26024: f64, t1860: f64, t2110: f64, t22549: f64, t24514: f64, t24517: f64, t26009: f64, t26016: f64, t26028: f64, t26070: f64, t26073: f64, t26076: f64, t6486: f64, t7256: f64, t7259: f64, t7428: f64, t7978: f64) -> (f64, f64, f64, f64, f64) {
    let t27298 = t2109 * t26012;
    let t27303 = t7974 * t6509;
    let t27308 = t7255 * t7445;
    let t27311 = t2109 * t26024;
    let t27326 = -5.0_f64 * t24514 * t26009 - 5.0_f64 / 3.0_f64 * t22549 * t27298 - 5.0_f64 / 3.0_f64 * t26016 * t24517 - t1860 * t27303 / 6.0_f64 - t6486 * t7978 / 6.0_f64 - t1860 * t27308 / 6.0_f64 - t1860 * t27311 / 6.0_f64 - t26028 * t2110 / 6.0_f64 - t7428 * t7256 / 6.0_f64 - t7428 * t7259 / 6.0_f64 + t26070 * t2110 / 3.0_f64 + t26073 * t2110 / 3.0_f64 + t26076 * t2110 / 3.0_f64;
    (t27298, t27303, t27308, t27311, t27326)
}
