//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 540/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk540(t1597: f64, t4513: f64, t3806: f64, t1557: f64, t3774: f64, t3780: f64, t3789: f64, t3793: f64, t3801: f64, t3808: f64, t3810: f64, t3910: f64, t3917: f64, t3920: f64, t4347: f64, t4351: f64, t4495: f64, t548: f64) -> (f64, f64) {
    let t4514 = t4513 * t1597;
    let t4519 = 0.38691203703703703703e-3_f64 * t3806;
    let t4527 = 0.15476481481481481481e-2_f64 * t3774 - 0.38691203703703703703e-3_f64 * t3780 + 0.34822083333333333332e-2_f64 * t3789 + 0.92858888888888888886e-2_f64 * t3793 + 0.74498e-1_f64 * t4347 * t4351 - 0.193e0_f64 * t1557 * t4514 - 0.23214722222222222222e-2_f64 * t3801 + t4495 * t548 - t4519 - 0.61905925925925925925e-2_f64 * t3808 + 0.23214722222222222222e-2_f64 * t3810 + 0.17411041666666666666e-2_f64 * t3910 + 0.17024129629629629629e-1_f64 * t3917 - 0.92858888888888888886e-2_f64 * t3920 + 0.193e0_f64 * t1557 * t4351;
    (t4514, t4527)
}
