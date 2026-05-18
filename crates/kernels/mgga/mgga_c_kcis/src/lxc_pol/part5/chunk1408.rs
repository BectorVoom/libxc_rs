//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1408/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1408<F: Float>(t23113: F, t23151: F, t23172: F, t23215: F, t1616: F, t1592: F, t22221: F, t22226: F, t22229: F, t22231: F, t22233: F, t22238: F, t22783: F, t4409: F, t4414: F, t6189: F, t6193: F, t7498: F, t7510: F) -> F {
    let t23217 = t23113 + t23151 + t23172 + t23215;
    let t23218 = t23217 * t1616;
    let t23227 = F::new(0.66725e-1) * t4409 * t7510 - F::new(0.17024129629629629629e-1) * t22221 + F::new(0.11349419753086419753e-1) * t22226 - F::new(0.61905925925925925925e-2) * t22229 - F::new(0.11607361111111111111e-2) * t22231 - F::new(0.66725e-1) * t4409 * t7498 - F::new(0.66725e-1) * t1592 * t23218 - F::new(0.13345e0) * t6193 * t6189 + F::new(0.178089025e-1) * t4414 * t22783 + F::new(0.15476481481481481481e-2) * t22233 - F::new(0.61905925925925925924e-2) * t22238;
    t23227
}
