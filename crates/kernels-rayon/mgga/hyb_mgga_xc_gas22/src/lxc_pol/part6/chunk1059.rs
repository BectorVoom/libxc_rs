//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1059/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1059(t1819: f64, t3819: f64, t555: f64, t3823: f64, t1782: f64, t3814: f64, t1787: f64, t1179: f64, t7913: f64, t7920: f64, t2997: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10102 = t555 * t1819 * t3819;
    let t10105 = t555 * t1819 * t3823;
    let t10107 = t1782 * t3814;
    let t10111 = t1787 * t3814;
    let t10115 = t7913 * t1179;
    let t10119 = t7920 * t1179;
    let t10123 = t2997 * t3;
    (t10102, t10105, t10107, t10111, t10115, t10119, t10123)
}
