//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 546/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk546<F: Float>(t1896: F, t4811: F, t1901: F, t1862: F, t1871: F, t1895: F, t1869: F, t1691: F, t670: F, t604: F, t1790: F, t667: F, t1689: F, t1692: F, t172: F, t342: F, t569: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4812 = t4811 * t1896;
    let t4814 = t4811 * t1901;
    let t4816 = t1862 * t1871;
    let t4817 = t4816 * sigma2;
    let t4818 = t4817 * t1895;
    let t4819 = t1869 * t4818;
    let t4822 = 1.0 / t1691 / t670;
    let t4823 = t604 * t4822;
    let t4824 = t1790 * t1790;
    let t4825 = t667 * t667;
    let t4826 = 1.0 / t4825;
    let t4827 = t4824 * t4826;
    let t4830 = t1689 * t1692;
    let t4834 = t342 * t172 * t569;
    (t4812, t4814, t4816, t4817, t4818, t4819, t4822, t4823, t4824, t4825, t4826, t4827, t4830, t4834)
}
