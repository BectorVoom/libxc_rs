//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1861/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861<F: Float>(t2109: F, t26012: F, t6509: F, t7974: F, t7255: F, t7445: F, t26024: F, t1860: F, t2110: F, t22549: F, t24514: F, t24517: F, t26009: F, t26016: F, t26028: F, t26070: F, t26073: F, t26076: F, t6486: F, t7256: F, t7259: F, t7428: F, t7978: F) -> (F, F, F, F, F) {
    let t27298 = t2109 * t26012;
    let t27303 = t7974 * t6509;
    let t27308 = t7255 * t7445;
    let t27311 = t2109 * t26024;
    let t27326 = -F::cast_from(5.0_f64) * t24514 * t26009 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22549 * t27298 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26016 * t24517 - t1860 * t27303 / F::cast_from(6.0_f64) - t6486 * t7978 / F::cast_from(6.0_f64) - t1860 * t27308 / F::cast_from(6.0_f64) - t1860 * t27311 / F::cast_from(6.0_f64) - t26028 * t2110 / F::cast_from(6.0_f64) - t7428 * t7256 / F::cast_from(6.0_f64) - t7428 * t7259 / F::cast_from(6.0_f64) + t26070 * t2110 / F::cast_from(3.0_f64) + t26073 * t2110 / F::cast_from(3.0_f64) + t26076 * t2110 / F::cast_from(3.0_f64);
    (t27298, t27303, t27308, t27311, t27326)
}
