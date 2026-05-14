//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1055/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1055<F: Float>(t14609: F, t1557: F, t21988: F, t22002: F, t26919: F, t26922: F, t26925: F, t26930: F, t26934: F, t26936: F, t26942: F, t26948: F, t26952: F, t26956: F, t26960: F, t26962: F, t26964: F, t26967: F, t26970: F, t27959: F, t27987: F) -> (F,) {
    let t28013 = 0.15476481481481481481e-2 * t26919 - 0.61905925925925925925e-2 * t26922 - 0.23214722222222222222e-2 * t26925 - 0.10446625e-1 * t26930 - t21988 - 0.46429444444444444443e-2 * t26934 + 0.11607361111111111111e-2 * t26936 - 0.15476481481481481481e-2 * t26942 - 0.51588271604938271605e-2 * t26948 + 0.51588271604938271605e-2 * t26952 + 0.77382407407407407408e-2 * t26956 + 0.77382407407407407407e-3 * t26960 - 0.23214722222222222221e-2 * t26962 + 0.15476481481481481481e-2 * t26964 - 0.34822083333333333332e-2 * t26967 - 0.61905925925925925925e-2 * t26970 - 0.43134342e-1 * t14609 * t27959 + 0.386e0 * t1557 * t27987 - t22002;
    (t28013,)
}
