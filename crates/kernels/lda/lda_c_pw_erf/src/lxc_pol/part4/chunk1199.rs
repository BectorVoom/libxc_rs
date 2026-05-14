//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1199/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1199<F: Float>(t16452: F, t4776: F, t571: F, t1472: F, t6385: F, t17685: F, t17688: F, t17691: F, t17693: F, t17695: F, t17698: F, t17700: F, t17705: F, t17707: F, t17710: F, t17714: F, t17716: F, t17719: F, t17721: F, t17724: F) -> (F, F, F) {
    let t17727 = 128.0 / 27.0 * t571 * t4776 * t16452;
    let t17729 = 64.0 / 81.0 * t1472 * t6385;
    let t17730 = -t17685 - t17688 + t17691 - t17693 + t17695 + t17698 + t17700 - t17705 - t17707 + t17710 - t17714 + t17716 - t17719 - t17721 - t17724 - t17727 + t17729;
    (t17727, t17729, t17730)
}
