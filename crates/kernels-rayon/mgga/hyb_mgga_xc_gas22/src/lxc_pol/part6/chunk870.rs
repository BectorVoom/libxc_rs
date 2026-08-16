//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 870/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk870(t7237: f64, t7238: f64, t7241: f64, t1110: f64, t2696: f64, t483: f64, t1112: f64, t1096: f64, t2635: f64, t2634: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7242 = t7237 * t7238 * t7241;
    let t7244 = 0.10254018858216406658e4_f64 * t1110 * t7242;
    let t7245 = t2696 * t483;
    let t7246 = t7245 * t1112;
    let t7249 = t2635 * t7238 * t1096;
    let t7251 = 0.35089341735807877242e1_f64 * t1110 * t7249;
    let t7253 = 1.0_f64 / t2634 / t488;
    (t7242, t7244, t7245, t7246, t7249, t7251, t7253)
}
