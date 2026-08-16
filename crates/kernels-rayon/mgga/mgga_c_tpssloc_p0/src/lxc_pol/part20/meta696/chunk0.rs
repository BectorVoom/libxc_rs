//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2654/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654(t12283: f64, t16271: f64, t16383: f64, t16370: f64, t16060: f64, t3798: f64, t1354: f64, t12345: f64, t5310: f64, t12339: f64, t16150: f64, t3866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54114 = t12283 * t16271;
    let t54116 = t12283 * t16383;
    let t54118 = t12283 * t16370;
    let t54124 = t16060 * t3798;
    let t54125 = t54124 * t1354;
    let t54131 = t12345 * t5310;
    let t54132 = 595.0_f64 / 1152.0_f64 * t54131;
    let t54133 = t12339 * t5310;
    let t54135 = t3866 * t16150;
    (t54114, t54116, t54118, t54125, t54132, t54133, t54135)
}
