//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 942/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk942(t2014: f64, t3283: f64, t684: f64, t1318: f64, t763: f64, t675: f64, t2002: f64, t3282: f64, t2028: f64, t1243: f64, t6469: f64, t1240: f64, t2011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8560 = t684 * t2014 * t3283 / 96.0_f64;
    let t8561 = t763 * t1318;
    let t8562 = t8561 * t675;
    let t8566 = t3282 * t2002;
    let t8570 = t3282 * t2028;
    let t8575 = t684 * t6469 * t1243;
    let t8577 = t1240 * t2011;
    (t8560, t8561, t8562, t8566, t8570, t8575, t8577)
}
