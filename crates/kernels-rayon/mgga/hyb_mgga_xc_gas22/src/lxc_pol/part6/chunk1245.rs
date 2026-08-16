//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1245/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1245(t2599: f64, t260: f64, t7109: f64, t1409: f64, t7075: f64, t3513: f64, t7150: f64, t2559: f64, t3524: f64, t1414: f64, t7058: f64, t2576: f64, t3557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25432 = t260 * t2599;
    let t25436 = t260 * t7109;
    let t25468 = t7075 * t1409;
    let t25520 = t3513 * t7150;
    let t25556 = t3524 * t2559;
    let t25561 = t1414 * t7058;
    let t25624 = t3557 * t2576;
    (t25432, t25436, t25468, t25520, t25556, t25561, t25624)
}
