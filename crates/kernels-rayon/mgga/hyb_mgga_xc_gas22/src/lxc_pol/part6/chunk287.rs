//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 287/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk287(t935: f64, t961: f64, t942: f64, t953: f64, t958: f64, t965: f64) -> (f64, f64, f64) {
    let t981 = 0.516475e0_f64 * t935;
    let t984 = 0.104195e0_f64 * t961;
    let t986 = 0.3529725e1_f64 * t953 - t981 + 0.1549425e1_f64 * t942 + 0.6311625e0_f64 * t958 - t984 + 0.312585e0_f64 * t965;
    (t981, t984, t986)
}
