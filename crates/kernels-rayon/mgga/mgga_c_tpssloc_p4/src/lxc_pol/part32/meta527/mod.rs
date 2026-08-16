//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta527(t2109: f64, t26012: f64, t6509: f64, t7974: f64, t7255: f64, t7445: f64, t26024: f64, t1860: f64, t2110: f64, t22549: f64, t24514: f64, t24517: f64, t26009: f64, t26016: f64, t26028: f64, t26070: f64, t26073: f64, t26076: f64, t6486: f64, t7256: f64, t7259: f64, t7428: f64, t7978: f64, t33: f64, t7973: f64, t2240: f64, t12571: f64, t7245: f64, t1419: f64, t55: f64, t22510: f64, t24498: f64, t3961: f64, t3966: f64, t607: f64, t7251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27298, t27303, t27308, t27311, t27326) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1861(t2109, t26012, t6509, t7974, t7255, t7445, t26024, t1860, t2110, t22549, t24514, t24517, t26009, t26016, t26028, t26070, t26073, t26076, t6486, t7256, t7259, t7428, t7978);
        let (t27331, t27332, t27341, t27356, t27363) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1862(t33, t7973, t2240, t12571, t7245, t1419, t55, t22510, t24498, t3961, t3966, t607, t7251);
    (t27298, t27303, t27308, t27311, t27326, t27331, t27332, t27341, t27356, t27363)
}
