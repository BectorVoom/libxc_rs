//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 696/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk696(t1422: f64, t986: f64, t2457: f64, t2505: f64, t2545: f64, t2550: f64, t3461: f64, t3472: f64, t3486: f64, t3491: f64, t3497: f64, t3499: f64, t3503: f64, t3507: f64, t3511: f64) -> (f64, f64) {
    let t3532 = t1422 * t986;
    let t3546 = -0.17648625e1_f64 * t3486 + 0.3529725e1_f64 * t3491 + t2545 - 0.516475e0_f64 * t2457 - 0.516475e0_f64 * t3461 + 0.1549425e1_f64 * t3472 + 0.31558125e0_f64 * t3497 + 0.6311625e0_f64 * t3499 + t2550 - 0.20839e0_f64 * t2505 - 0.20839e0_f64 * t3503 + 0.312585e0_f64 * t3507 + 0.312585e0_f64 * t3511;
    (t3532, t3546)
}
