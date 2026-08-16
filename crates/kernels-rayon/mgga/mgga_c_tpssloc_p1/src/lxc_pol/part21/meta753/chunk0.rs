//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2627/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2627(t12283: f64, t16248: f64, t40138: f64, t5293: f64, t16275: f64, t16271: f64, t16383: f64, t16370: f64, t16060: f64, t3798: f64, t1354: f64, t12345: f64, t5310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54088 = t12283 * t16248;
    let t54090 = t40138 * t5293;
    let t54092 = t12283 * t16275;
    let t54114 = t12283 * t16271;
    let t54116 = t12283 * t16383;
    let t54118 = t12283 * t16370;
    let t54124 = t16060 * t3798;
    let t54125 = t54124 * t1354;
    let t54131 = t12345 * t5310;
    (t54088, t54090, t54092, t54114, t54116, t54118, t54124, t54125, t54131)
}
