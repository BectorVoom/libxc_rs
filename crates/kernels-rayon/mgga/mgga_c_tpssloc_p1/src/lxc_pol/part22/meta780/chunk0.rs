//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2670/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2670(t56390: f64, t56394: f64, t56398: f64, t56400: f64, t54432: f64, t54434: f64, t39570: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t54429: f64, t54430: f64, t54431: f64, t54436: f64, t54437: f64, t54438: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t74481 = 36.0_f64 * t56390;
    let t74482 = 60.0_f64 * t56394;
    let t74483 = 3.0_f64 * t56398;
    let t74484 = 3.0_f64 * t56400;
    let t74485 = 180.0_f64 * t54432;
    let t74486 = 0.15584273195113317383e3_f64 * t54434;
    let t74487 = -t54429 + t39570 - t74481 + t74482 - t54430 + t74483 - t54431 + t74484 - t39585 + t39590 - t39593 + t39595 + t74485 - t74486 - t54436 + t54437 - t54438;
    (t74481, t74482, t74483, t74484, t74485, t74486, t74487)
}
