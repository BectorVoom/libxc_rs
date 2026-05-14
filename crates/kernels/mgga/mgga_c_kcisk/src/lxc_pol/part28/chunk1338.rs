//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1338/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1338<F: Float>(t116393: F, t34580: F, t9736: F, t116423: F, t18682: F, t964: F, t62760: F, t79: F, t33183: F, t34484: F, t33257: F, t9999: F, t117840: F, t9740: F, t117898: F, t2804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t117913 = 0.25794135802469135802e-2 * t116393;
    let t117925 = t34580 * t9736;
    let t117927 = 0.15476481481481481481e-2 * t116423;
    let t117934 = t964 * t18682;
    let t117951 = t62760 * t79;
    let t117967 = 0.13402777777777777778e-2 * t33183 * t34484;
    let t118003 = t33257 * t9999;
    let t118021 = 0.11574074074074074074e-2 * t9740 * t117840;
    let t118032 = 0.34722222222222222222e-2 * t2804 * t117898;
    (t117913, t117925, t117927, t117934, t117951, t117967, t118003, t118021, t118032)
}
