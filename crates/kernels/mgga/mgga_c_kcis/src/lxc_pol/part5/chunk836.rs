//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 836/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk836<F: Float>(t1616: F, t7497: F, t2109: F, t4314: F, t1592: F, t4414: F, t5681: F, t5684: F, t5686: F, t6906: F, t6910: F, t6915: F, t6920: F, t6925: F, t6930: F, t6934: F, t7031: F, t7035: F, t7039: F) -> (F, F, F, F) {
    let t7498 = t7497 * t1616;
    let t7509 = t2109 * t2109;
    let t7510 = t7509 * t4314;
    let t7515 = 0.15476481481481481481e-2 * t6906 - 0.61905925925925925925e-2 * t6910 - 0.23214722222222222222e-2 * t6915 - 0.66725e-1 * t1592 * t7498 - 0.23214722222222222222e-2 * t6920 - 0.38691203703703703703e-3 * t6925 + 0.34822083333333333332e-2 * t6930 + 0.92858888888888888886e-2 * t6934 + 0.23214722222222222222e-2 * t7031 + 0.11607361111111111111e-2 * t7035 + 0.19345601851851851852e-2 * t7039 + 0.15476481481481481481e-2 * t5681 + 0.890445125e-2 * t4414 * t7510 - 0.61905925925925925925e-2 * t5684 + 0.23214722222222222222e-2 * t5686;
    (t7498, t7509, t7510, t7515)
}
