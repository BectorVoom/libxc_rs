//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2168/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168(t54555: f64, t12289: f64, t1336: f64, t836: f64, t1811: f64, t40005: f64, t40281: f64, t5259: f64, t1361: f64, t242: f64, t12189: f64, t5206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54556 = 119.0_f64 / 4608.0_f64 * t54555;
    let t54566 = t1336 * t12289 * t836;
    let t54582 = t40005 * t1811;
    let t54611 = t40281 * t5259;
    let t54612 = 119.0_f64 / 1152.0_f64 * t54611;
    let t54614 = t1336 * t1361 * t242;
    let t54631 = t12189 * t5206;
    (t54556, t54566, t54582, t54612, t54614, t54631)
}
