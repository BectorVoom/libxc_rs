//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2689/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2689(t54647: f64, t54658: f64, t54687: f64, t54736: f64, t225: f64, t1336: f64, t242: f64, t40042: f64, t12177: f64, t40046: f64, t16391: f64, t16398: f64) -> (f64, f64, f64, f64, f64) {
    let t54738 = t54647 + t54658 + t54687 + t54736;
    let t54739 = t54738 * t225;
    let t54744 = t1336 * t40042 * t242;
    let t54745 = t40046 * t12177;
    let t54750 = t16398 * t16391;
    (t54738, t54739, t54744, t54745, t54750)
}
