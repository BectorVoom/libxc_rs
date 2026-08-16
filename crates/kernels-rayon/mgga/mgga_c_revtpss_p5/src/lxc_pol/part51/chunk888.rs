//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 888/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk888(t4181: f64, t603: f64, t4187: f64, t38: f64, t7714: f64, t2247: f64, t1493: f64, t644: f64, t77: f64, t13272: f64, t6957: f64, t4173: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28116 = t603 * t4181;
    let t28119 = t603 * t4187;
    let t28126 = t38 * t7714;
    let t28127 = t2247 * t28126;
    let t28133 = t77 * t1493 * t644;
    let t28138 = t13272 * t6957;
    let t28141 = t4173 * t607;
    (t28116, t28119, t28127, t28133, t28138, t28141)
}
