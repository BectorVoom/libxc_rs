//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1407/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1407(t114992: f64, t115009: f64, t119700: f64, t121258: f64, t121271: f64, t121279: f64, t121775: f64, t121782: f64, t121818: f64, t1877: f64, t23788: f64, t24191: f64, t2522: f64, t25921: f64, t25928: f64, t26563: f64, t26744: f64, t26756: f64, t28: f64, t31504: f64, t33466: f64, t6841: f64, t6848: f64, t7656: f64, t89953: f64) -> f64 {
    let t122042 = -3.0_f64 * t26563 * t23788 * t121818 + t26756 * t119700 + 3.0_f64 / 2.0_f64 * t2522 * t33466 * t6841 - 3.0_f64 * t26756 * t89953 * t121258 - t1877 * t121782 * t6848 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t23788 * t121279 - 3.0_f64 / 2.0_f64 * t115009 * t25921 - t1877 * t26744 * t31504 / 2.0_f64 + t121271 * t25928 + t1877 * t121775 * t28 / 2.0_f64 - t1877 * t114992 * t7656 / 2.0_f64;
    t122042
}
