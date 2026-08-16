//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2636/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2636(t16235: f64, t54566: f64, t1811: f64, t40005: f64, t12283: f64, t16265: f64, t16257: f64, t16398: f64, t1358: f64, t16347: f64, t40281: f64, t5259: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54567 = t54566 * t16235;
    let t54582 = t40005 * t1811;
    let t54585 = t12283 * t16265;
    let t54607 = t16398 * t16257;
    let t54609 = t16347 * t1358;
    let t54611 = t40281 * t5259;
    (t54567, t54582, t54585, t54607, t54609, t54611)
}
