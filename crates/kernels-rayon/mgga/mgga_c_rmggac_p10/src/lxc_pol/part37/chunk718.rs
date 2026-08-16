//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 718/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk718(t3047: f64, t49: f64, t35688: f64, t7935: f64, t14362: f64, t2190: f64, t3144: f64, t25561: f64, t29: f64, t3117: f64, t3132: f64, t3136: f64) -> (f64, f64, f64, f64, f64) {
    let t70171 = t3047 * t49;
    let t70173 = t35688 * t70171 * t7935;
    let t70176 = t2190 * t14362 * t3144;
    let t70186 = t3117 * t25561 * t29;
    let t70188 = t3132 * t70186 * t3136;
    (t70171, t70173, t70176, t70186, t70188)
}
