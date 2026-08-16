//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 898/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk898(t2240: f64, t31680: f64, t1862: f64, t31: f64, t607: f64, t8308: f64, t625: f64, t8301: f64, t8515: f64, t79: f64, t641: f64, t8513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31681 = t2240 * t31680;
    let t31682 = t1862 * t31;
    let t31683 = t31682 * t607;
    let t31684 = t8308 * t31683;
    let t31687 = t8301 * t625;
    let t31688 = t2240 * t31687;
    let t31690 = 5.0_f64 / 27.0_f64 * t31688 * t8515;
    let t31691 = t79 * t1862;
    let t31693 = t8513 * t31691 * t641;
    (t31681, t31682, t31684, t31687, t31688, t31690, t31691, t31693)
}
