//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1079/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1079(t1873: f64, t649: f64, t6534: f64, t89: f64, t88: f64, t1458: f64, t8439: f64, t4028: f64, t8323: f64, t7458: f64, t7670: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31717 = t88 * t6534;
    let t32656 = t8439 * t1458;
    let t32659 = t4028 * t8323;
    let t32661 = t7458 * t8323;
    let t32663 = t7670 * t1873;
    let t32664 = t652 * t32663;
    (t31537, t31540, t31717, t32656, t32659, t32661, t32663, t32664)
}
