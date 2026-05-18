//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1329/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1329<F: Float>(t10617: F, t2183: F, t20691: F, t20697: F, t28794: F, t28797: F, t28800: F, t28804: F, t28808: F, t28837: F, t28840: F, t28844: F, t28847: F) -> (F, F) {
    let t28973 = F::new(2.0) * t2183 * t10617;
    let t28985 = -F::new(0.1460562962962962963e1) * t20691 + F::new(0.27385555555555555556e0) * t20697 + F::new(0.27385555555555555555e0) * t28794 - F::new(0.32862666666666666666e0) * t28797 - F::new(0.32862666666666666666e0) * t28800 + F::new(0.24647e0) * t28804 + F::new(0.49294e0) * t28808 + F::new(0.24647e0) * t28837 - F::new(0.65725333333333333333e0) * t28840 + F::new(0.49294e0) * t28844 - F::new(0.1898925e1) * t28847;
    (t28973, t28985)
}
