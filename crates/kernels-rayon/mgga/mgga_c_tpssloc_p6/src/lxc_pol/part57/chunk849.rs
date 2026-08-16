//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 849/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk849(t1862: f64, t79: f64, t532: f64, t8639: f64, t112: f64, t8646: f64, t4028: f64, t8327: f64, t7458: f64, t1774: f64, t8326: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31691 = t79 * t1862;
    let t31758 = t532 * t8639;
    let t31795 = t8646 * t112;
    let t32673 = t4028 * t8327;
    let t32674 = 2.0_f64 * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = 2.0_f64 * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    (t31691, t31758, t31795, t32673, t32674, t32675, t32676, t32677, t32678)
}
