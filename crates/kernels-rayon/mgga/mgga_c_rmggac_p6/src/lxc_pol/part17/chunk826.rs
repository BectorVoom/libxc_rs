//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 826/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk826(t1618: f64, t1986: f64, t1600: f64, t7487: f64, t8352: f64, t534: f64, t7350: f64, t7349: f64, t7353: f64, t4617: f64, t507: f64, t1622: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40702 = t1986 * t1618;
    let t40705 = t1986 * t1600;
    let t40715 = t7487 * t8352;
    let t40716 = 0.19211284388664477842e-2_f64 * t40715;
    let t40717 = t7350 * t534;
    let t40719 = t7349 * t40717 * t7353;
    let t40724 = t507 * t4617;
    let t40731 = t1986 * t1622;
    (t40702, t40705, t40716, t40719, t40724, t40731)
}
