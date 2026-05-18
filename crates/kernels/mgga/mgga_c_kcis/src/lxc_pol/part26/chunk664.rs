//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 664/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk664<F: Float>(t4314: F, t7509: F, t1592: F, t4414: F, t5681: F, t5684: F, t5686: F, t6906: F, t6910: F, t6915: F, t6920: F, t6925: F, t6930: F, t6934: F, t7031: F, t7035: F, t7039: F, t7498: F) -> (F, F) {
    let t7510 = t7509 * t4314;
    let t7515 = F::new(0.15476481481481481481e-2) * t6906 - F::new(0.61905925925925925925e-2) * t6910 - F::new(0.23214722222222222222e-2) * t6915 - F::new(0.66725e-1) * t1592 * t7498 - F::new(0.23214722222222222222e-2) * t6920 - F::new(0.38691203703703703703e-3) * t6925 + F::new(0.34822083333333333332e-2) * t6930 + F::new(0.92858888888888888886e-2) * t6934 + F::new(0.23214722222222222222e-2) * t7031 + F::new(0.11607361111111111111e-2) * t7035 + F::new(0.19345601851851851852e-2) * t7039 + F::new(0.15476481481481481481e-2) * t5681 + F::new(0.890445125e-2) * t4414 * t7510 - F::new(0.61905925925925925925e-2) * t5684 + F::new(0.23214722222222222222e-2) * t5686;
    (t7510, t7515)
}
