//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1019/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1019(t2867: f64, t3756: f64, t532: f64, t2824: f64, t3705: f64, t3687: f64, t531: f64, t1143: f64, t3697: f64, t1159: f64, t7636: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9538 = t2867 * t3756;
    let t9542 = t2867 * t532;
    let t9545 = t3705 * t2824;
    let t9548 = t3687 * t531;
    let t9549 = t1143 * t9548;
    let t9552 = t3697 * t2824;
    let t9557 = t7636 * t1159;
    let t9558 = t524 * t9557;
    (t9538, t9542, t9545, t9548, t9549, t9552, t9557, t9558)
}
