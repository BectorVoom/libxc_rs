//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 743/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk743<F: Float>(t40715: F, t534: F, t7350: F, t7349: F, t7353: F, t4617: F, t507: F, t1622: F, t1986: F, t275: F, t9064: F, t1679: F, t7197: F, t34760: F, t9221: F, t352: F, t8915: F) -> (F, F, F, F, F, F, F, F) {
    let t40716 = 0.19211284388664477842e-2 * t40715;
    let t40717 = t7350 * t534;
    let t40719 = t7349 * t40717 * t7353;
    let t40724 = t507 * t4617;
    let t40731 = t1986 * t1622;
    let t40750 = 2.0 * t275 * t9064;
    let t40759 = t1679 * t7197;
    let t40771 = t9221 * t34760;
    let t40802 = t8915 * t352;
    (t40716, t40719, t40724, t40731, t40750, t40759, t40771, t40802)
}
