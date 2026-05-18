//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1216/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1216<F: Float>(t14104: F, t14567: F, t14576: F, t10255: F, t10257: F, t10450: F, t10452: F, t10473: F, t11209: F, t14095: F, t14100: F, t14102: F, t14108: F, t14113: F, t14377: F, t14384: F, t14388: F, t14390: F, t14574: F, t15611: F, t3644: F) -> F {
    let t15648 = F::new(0.15476481481481481481e-2) * t14104;
    let t15659 = F::new(0.23214722222222222222e-2) * t14567;
    let t15662 = F::new(0.15476481481481481481e-2) * t14576;
    let t15663 = -F::new(0.61905925925925925926e-2) * t10255 + F::new(0.11349419753086419753e-1) * t10257 + F::new(0.69644166666666666664e-2) * t14095 + F::new(0.34822083333333333332e-2) * t14100 + F::new(0.46429444444444444443e-2) * t14102 - t15648 + F::new(0.20635308641975308642e-2) * t14108 - F::new(0.38691203703703703703e-3) * t14113 - F::new(0.2671335375e-1) * t3644 * t15611 - F::new(0.17411041666666666666e-2) * t14377 - F::new(0.77382407407407407406e-3) * t10450 + F::new(0.11607361111111111111e-2) * t10452 + F::new(0.46429444444444444444e-2) * t14384 - F::new(0.38691203703703703704e-2) * t14388 - F::new(0.25794135802469135802e-3) * t14390 + t15659 + t11209 + F::new(0.20635308641975308642e-2) * t10473 + F::new(0.19345601851851851852e-2) * t14574 - t15662;
    t15663
}
