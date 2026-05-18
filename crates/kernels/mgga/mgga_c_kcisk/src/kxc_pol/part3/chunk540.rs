//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 540/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk540<F: Float>(t1597: F, t4513: F, t3806: F, t1557: F, t3774: F, t3780: F, t3789: F, t3793: F, t3801: F, t3808: F, t3810: F, t3910: F, t3917: F, t3920: F, t4347: F, t4351: F, t4495: F, t548: F) -> (F, F) {
    let t4514 = t4513 * t1597;
    let t4519 = F::new(0.38691203703703703703e-3) * t3806;
    let t4527 = F::new(0.15476481481481481481e-2) * t3774 - F::new(0.38691203703703703703e-3) * t3780 + F::new(0.34822083333333333332e-2) * t3789 + F::new(0.92858888888888888886e-2) * t3793 + F::new(0.74498e-1) * t4347 * t4351 - F::new(0.193e0) * t1557 * t4514 - F::new(0.23214722222222222222e-2) * t3801 + t4495 * t548 - t4519 - F::new(0.61905925925925925925e-2) * t3808 + F::new(0.23214722222222222222e-2) * t3810 + F::new(0.17411041666666666666e-2) * t3910 + F::new(0.17024129629629629629e-1) * t3917 - F::new(0.92858888888888888886e-2) * t3920 + F::new(0.193e0) * t1557 * t4351;
    (t4514, t4527)
}
