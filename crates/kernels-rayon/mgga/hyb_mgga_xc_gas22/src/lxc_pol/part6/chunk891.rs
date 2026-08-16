//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 891/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk891(t1139: f64, t2867: f64, t1143: f64, t2874: f64, t1166: f64, t2880: f64, t526: f64, t530: f64, t1128: f64, t2938: f64, t511: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7721 = t2867 * t1139;
    let t7734 = t1143 * t2874;
    let t7739 = t1166 * t2880;
    let t7744 = 1.0_f64 / t530 / t526 / 2.0_f64;
    let t7764 = t2938 * t1128;
    let t7768 = 1.0_f64 / t519 / t511;
    (t7721, t7734, t7739, t7744, t7764, t7768)
}
