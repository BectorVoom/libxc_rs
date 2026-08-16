//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1048/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1048(t1890: f64, t3832: f64, t3814: f64, t6025: f64, t545: f64, t7945: f64, t6033: f64, t3008: f64, t3: f64, t3009: f64, t3836: f64, t1897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9870 = t1890 * t3832;
    let t9872 = t6025 * t3814;
    let t9874 = t7945 * t9872 * t545;
    let t9877 = t6033 * t3814;
    let t9879 = t3008 * t9877 * t545;
    let t9883 = t3008 * t3009 * t3;
    let t9886 = t1890 * t3836;
    let t9888 = t1897 * t3814;
    (t9870, t9872, t9874, t9877, t9879, t9883, t9886, t9888)
}
