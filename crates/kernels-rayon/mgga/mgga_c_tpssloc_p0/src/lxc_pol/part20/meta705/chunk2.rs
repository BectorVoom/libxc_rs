//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2680/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2680(t12283: f64, t16265: f64, t1351: f64, t3719: f64, t16257: f64, t16398: f64, t1358: f64, t16347: f64, t40281: f64, t5259: f64, t1336: f64, t1361: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54585 = t12283 * t16265;
    let t54591 = t3719 * t1351;
    let t54607 = t16398 * t16257;
    let t54609 = t16347 * t1358;
    let t54611 = t40281 * t5259;
    let t54612 = 119.0_f64 / 1152.0_f64 * t54611;
    let t54614 = t1336 * t1361 * t242;
    (t54585, t54591, t54607, t54609, t54612, t54614)
}
