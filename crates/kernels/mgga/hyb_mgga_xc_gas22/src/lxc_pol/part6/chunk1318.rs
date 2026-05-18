//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1318/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1318<F: Float>(t226: F, t238: F, t242: F, t28834: F, t10600: F, t801: F, t1329: F, t8646: F, t3329: F, t2194: F, t20691: F, t20697: F, t28794: F, t28797: F, t28800: F, t28804: F, t28808: F) -> (F, F, F, F, F, F) {
    let t28837 = t238 * t242 * t226 * t28834;
    let t28840 = t238 * t801 * t10600;
    let t28844 = t238 * t242 * t1329 * t8646;
    let t28846 = t3329 * t3329;
    let t28847 = t2194 * t28846;
    let t28849 = -F::new(0.14717333333333333333e1) * t20691 + F::new(0.27595e0) * t20697 + F::new(0.27595e0) * t28794 - F::new(0.33114e0) * t28797 - F::new(0.33114e0) * t28800 + F::new(0.248355e0) * t28804 + F::new(0.49671e0) * t28808 + F::new(0.248355e0) * t28837 - F::new(0.66228e0) * t28840 + F::new(0.49671e0) * t28844 - F::new(0.258925e1) * t28847;
    (t28837, t28840, t28844, t28846, t28847, t28849)
}
