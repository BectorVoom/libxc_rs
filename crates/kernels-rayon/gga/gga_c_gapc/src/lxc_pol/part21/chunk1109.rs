//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1109/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1109(t11365: f64, t7294: f64, t7880: f64, t11897: f64, t9670: f64, t10058: f64, t11808: f64, t28472: f64, t3708: f64, t9574: f64, t11840: f64, t9520: f64) -> (f64, f64, f64, f64, f64) {
    let t33770 = t7294 * t11365 * t7880;
    let t33772 = t11897 * t9670;
    let t33774 = t11808 * t10058;
    let t33777 = t9574 * t3708 * t28472;
    let t33779 = t11840 * t9520;
    (t33770, t33772, t33774, t33777, t33779)
}
