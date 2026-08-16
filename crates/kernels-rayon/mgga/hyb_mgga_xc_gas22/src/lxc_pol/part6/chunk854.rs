//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 854/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk854(t2477: f64, t944: f64, t222: f64, t343: f64, t6007: f64, t1885: f64, t940: f64) -> (f64, f64, f64, f64) {
    let t6951 = t944 * t2477;
    let t6966 = t222 * t6007 * t343;
    let t6967 = 0.28842592592592592592e-1_f64 * t6966;
    let t6969 = t222 * t1885 * t940;
    (t6951, t6966, t6967, t6969)
}
