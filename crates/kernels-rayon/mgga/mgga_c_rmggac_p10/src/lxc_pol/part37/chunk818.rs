//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 818/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk818(t11675: f64, t14236: f64, t2067: f64, t68427: f64, t11679: f64, t70397: f64, t13806: f64, t8368: f64, t15379: f64, t68937: f64, t69904: f64, t8571: f64) -> (f64, f64, f64, f64, f64) {
    let t74734 = t14236 * t68427 * t2067 * t11675;
    let t74739 = t14236 * t70397 * t2067 * t11679;
    let t74741 = t8368 * t13806;
    let t74743 = t15379 * t68937;
    let t74745 = t8571 * t69904;
    (t74734, t74739, t74741, t74743, t74745)
}
