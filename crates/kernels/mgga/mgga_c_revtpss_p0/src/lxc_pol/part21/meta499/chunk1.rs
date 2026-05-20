//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2108/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2108<F: Float>(t15780: F, t4894: F, t3117: F, t3133: F, t3154: F, t4893: F, t13396: F, t4801: F, t1042: F, t11922: F, t4911: F, t3115: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15781 = t15780 * t4894;
    let t15782 = t3117 * t15781;
    let t15785 = t3154 * t3133;
    let t15786 = t4893 * t15785;
    let t15787 = t3117 * t15786;
    let t15790 = t4801 * t13396;
    let t15791 = t1042 * t15790;
    let t15794 = t11922 * t4911;
    let t15796 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t15794;
    (t15781, t15782, t15785, t15786, t15787, t15790, t15791, t15794, t15796)
}
