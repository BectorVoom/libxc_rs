//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1204/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1204<F: Float>(t28203: F, t3489: F, t15573: F, t28131: F, t7788: F, t96727: F, t27014: F, t28214: F, t95903: F, t11081: F, t26960: F, t28106: F) -> (F, F, F, F, F, F, F) {
    let t97015 = t28203 * t3489;
    let t97024 = t15573 * t28131;
    let t97026 = F::new(0.23168402777777777778e-3) * t7788 * t97024;
    let t97028 = F::new(0.46336805555555555556e-3) * t7788 * t96727;
    let t97030 = F::new(0.7722800925925925926e-4) * t27014 * t28214;
    let t97031 = F::new(0.15476481481481481481e-2) * t95903;
    let t97051 = F::new(0.7722800925925925926e-4) * t26960 * t11081 * t28106;
    (t97015, t97024, t97026, t97028, t97030, t97031, t97051)
}
