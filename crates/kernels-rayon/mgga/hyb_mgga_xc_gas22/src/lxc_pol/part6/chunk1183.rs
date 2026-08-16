//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1183/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1183(t2679: f64, t576: f64, t2687: f64, t700: f64, t1096: f64, t1110: f64, t21841: f64, t2647: f64, t2727: f64, t2730: f64, t21837: f64, t441: f64) -> (f64, f64, f64, f64) {
    let t21896 = 1.0_f64 / t2679 / t576;
    let t21911 = 1.0_f64 / t2687 / t700;
    let t21932 = 0.35089341735807877242e1_f64 * t1110 * t2647 * t21841 * t1096;
    let t21933 = t2727 * t2727;
    let t21936 = t2730 * t2730;
    let t21940 = 0.24955700379505800916e5_f64 * t441 / t21933 * t21837 / t21936;
    (t21896, t21911, t21932, t21940)
}
