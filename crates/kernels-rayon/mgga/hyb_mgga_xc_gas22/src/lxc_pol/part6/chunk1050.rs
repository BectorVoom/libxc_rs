//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1050/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1050(t575: f64, t578: f64, t9909: f64, t3023: f64, t572: f64, t6010: f64, t6013: f64, t7933: f64, t7936: f64, t7938: f64, t7943: f64, t9870: f64, t9874: f64, t9879: f64, t9883: f64, t9886: f64, t9890: f64, t9894: f64, t9897: f64, t9901: f64, t9906: f64) -> (f64, f64) {
    let t9911 = t575 * t578 * t9909;
    let t9914 = -t6010 - 2.0_f64 / 243.0_f64 * t6013 - 4.0_f64 / 243.0_f64 * t7933 + t7936 - t7938 + 2.0_f64 / 81.0_f64 * t7943 + t9870 / 243.0_f64 - 5.0_f64 / 243.0_f64 * t572 * t9874 + 2.0_f64 / 27.0_f64 * t572 * t9879 - 4.0_f64 / 81.0_f64 * t3023 * t9883 - t9886 / 81.0_f64 - t572 * t9890 / 9.0_f64 + 4.0_f64 / 27.0_f64 * t3023 * t9894 + t9897 / 162.0_f64 - t572 * t9901 / 81.0_f64 + t572 * t9906 / 27.0_f64 - t572 * t9911 / 54.0_f64;
    (t9911, t9914)
}
