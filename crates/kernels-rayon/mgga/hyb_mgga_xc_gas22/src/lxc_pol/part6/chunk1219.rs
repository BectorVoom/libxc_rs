//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1219/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1219(t2998: f64, t555: f64, t6160: f64, t1819: f64, t7914: f64, t7921: f64, t7925: f64, t125: f64, t8145: f64, t3112: f64, t668: f64, t23029: f64, t3124: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23756 = t555 * t6160 * t2998;
    let t23759 = t555 * t1819 * t7914;
    let t23762 = t555 * t1819 * t7921;
    let t23765 = t555 * t1819 * t7925;
    let t23767 = t8145 * t125;
    let t23772 = t3112 * t668;
    let t23783 = t23029 * t3124;
    (t23756, t23759, t23762, t23765, t23767, t23772, t23783)
}
