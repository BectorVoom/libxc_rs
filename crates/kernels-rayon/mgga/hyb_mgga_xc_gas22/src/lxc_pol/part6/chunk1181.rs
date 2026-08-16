//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1181/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1181(t1056: f64, t7336: f64, t20685: f64, t2655: f64, t2654: f64, t16: f64, t7940: f64, t1033: f64, t15: f64, t221: f64, t439: f64, t12: f64, t21862: f64, t222: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21866 = t7336 * t1056;
    let t21868 = t2655 * t20685;
    let t21869 = t2654 * t21868;
    let t21871 = t16 * t7940;
    let t21872 = t1033 * t21871;
    let t21874 = t15 * t7940;
    let t21875 = t221 * t21874;
    let t21877 = f64::powf(t439, -0.25e1_f64);
    let t21880 = t21877 * t12 * t21862 * t222;
    (t21866, t21868, t21869, t21871, t21872, t21874, t21875, t21880)
}
