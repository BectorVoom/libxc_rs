//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 602/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk602(t15: f64, t3318: f64, t20: f64, t398: f64, sigma2: f64) -> (f64, f64, f64) {
    let t3319 = t15 * t3318;
    let t3323 = 1.0_f64 / t20 / t398;
    let t3324 = sigma2 * t3323;
    (t3319, t3323, t3324)
}
