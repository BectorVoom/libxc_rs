//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1125/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1125(t134: f64, t332: f64, t7877: f64, t1038: f64, t18813: f64, t2579: f64, t2801: f64, t18822: f64, t3787: f64, t15515: f64, t7592: f64, t1: f64, t932: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28415 = t332 * t134;
    let t28416 = t28415 * t7877;
    let t28427 = t2579 * t2801 * t1038 * t18813;
    let t28472 = t3787 * t1038 * t18822;
    let t28517 = t7592 * t15515;
    let t28524 = t932 * t1;
    (t28415, t28416, t28427, t28472, t28517, t28524)
}
