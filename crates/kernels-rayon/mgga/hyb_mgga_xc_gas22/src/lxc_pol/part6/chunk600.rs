//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 600/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk600(t1828: f64, t1161: f64, t524: f64, t525: f64, t529: f64, t530: f64, sigma0: f64) -> (f64, f64, f64) {
    let t2841 = sigma0 * t1828;
    let t2842 = t1161 * t2841;
    let t2847 = t524 * t525;
    let t2849 = 1.0_f64 / t530 / t529;
    (t2842, t2847, t2849)
}
