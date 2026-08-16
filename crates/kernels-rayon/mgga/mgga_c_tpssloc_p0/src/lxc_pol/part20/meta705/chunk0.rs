//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2678/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2678(t12365: f64, t5289: f64, t1827: f64, t39955: f64, t16261: f64, t16398: f64, t12289: f64, t1336: f64, t836: f64, t16235: f64, t1811: f64, t40005: f64) -> (f64, f64, f64, f64, f64) {
    let t54555 = t12365 * t5289;
    let t54556 = 119.0_f64 / 4608.0_f64 * t54555;
    let t54557 = t39955 * t1827;
    let t54561 = t16398 * t16261;
    let t54566 = t1336 * t12289 * t836;
    let t54567 = t54566 * t16235;
    let t54582 = t40005 * t1811;
    (t54556, t54557, t54561, t54567, t54582)
}
