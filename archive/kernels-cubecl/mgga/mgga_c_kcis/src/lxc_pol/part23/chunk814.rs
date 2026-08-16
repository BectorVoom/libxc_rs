//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 814/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk814<F: Float>(t15828: F, t4170: F, t4160: F, t11862: F, t5668: F, t2046: F, t3797: F, t5661: F, t2038: F, t3805: F, t4162: F, t4142: F, t5773: F) -> (F, F, F, F, F, F, F) {
    let t15829 = t4170 * t15828;
    let t15830 = t4160 * t15829;
    let t15832 = t11862 * t5668;
    let t15834 = t2046 * t3797;
    let t15835 = t4170 * t15834;
    let t15836 = t5661 * t15835;
    let t15838 = t2038 * t3805;
    let t15839 = t4162 * t15838;
    let t15840 = t4160 * t15839;
    let t15844 = t4142 * t5773;
    (t15830, t15832, t15834, t15836, t15838, t15840, t15844)
}
