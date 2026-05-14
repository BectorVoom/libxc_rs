//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1357/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1357<F: Float>(t116210: F, t34559: F, t964: F, t33258: F, t34484: F, t10004: F, t33257: F, t10009: F, t112815: F, t112817: F, t113085: F, t116206: F, t116225: F, t116228: F, t15909: F, t33180: F, t33188: F, t34462: F, t34561: F, t34573: F, t9728: F, t9740: F, t9991: F) -> (F,) {
    let t117739 = 0.15476481481481481481e-2 * t116210;
    let t117740 = t964 * t34559;
    let t117751 = 0.13402777777777777778e-2 * t33258 * t34484;
    let t117752 = t33257 * t10004;
    let t117759 = -0.34722222222222222222e-2 * t113085 * t10009 - 0.23214722222222222222e-2 * t116206 - 0.34722222222222222222e-2 * t112815 + t117739 + 0.92592592592592592592e-2 * t9740 * t117740 * t34561 * t15909 + 0.11574074074074074074e-2 * t112817 + 0.52083333333333333333e-2 * t9991 * t33188 - 0.27777777777777777778e-1 * t34462 * t9728 + t117751 - 0.10722222222222222222e-1 * t117752 * t9728 - 0.23214722222222222222e-2 * t116225 + 0.15476481481481481481e-2 * t116228 + 0.16083333333333333333e-1 * t34573 * t33180;
    (t117759,)
}
