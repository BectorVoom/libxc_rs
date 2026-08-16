//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 960/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk960(t117496: f64, t1409: f64, t31864: f64, t8308: f64, t32344: f64, t33669: f64, t33677: f64, t1437: f64, t31860: f64, t32343: f64, t8513: f64, t117480: f64, t1433: f64, t8663: f64) -> (f64, f64, f64, f64, f64) {
    let t124803 = t31864 * t8308 * t117496 * t1409;
    let t124805 = t33669 * t32344;
    let t124807 = t33677 * t32344;
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    (t124803, t124805, t124807, t124834, t124838)
}
