//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 865/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk865<F: Float>(t7487: F, t8352: F, t534: F, t7350: F, t7349: F, t7353: F, t798: F, t8936: F, t4617: F, t507: F, t4048: F, t1622: F, t1986: F, t7720: F, t321: F, t8924: F) -> (F, F, F, F, F, F, F) {
    let t40715 = t7487 * t8352;
    let t40716 = 0.19211284388664477842e-2 * t40715;
    let t40717 = t7350 * t534;
    let t40719 = t7349 * t40717 * t7353;
    let t40721 = t8936 * t798;
    let t40724 = t507 * t4617;
    let t40725 = t8936 * t4048;
    let t40731 = t1986 * t1622;
    let t40732 = t7720 * t40731;
    let t40734 = t8924 * t321;
    (t40716, t40719, t40721, t40724, t40725, t40732, t40734)
}
