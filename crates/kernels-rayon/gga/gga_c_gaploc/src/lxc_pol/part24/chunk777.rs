//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 777/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk777(t7140: f64, t7196: f64, t7249: f64, t7320: f64, t1959: f64, t952: f64, t2728: f64, t841: f64, t2208: f64, t977: f64, t1402: f64, t2576: f64) -> (f64, f64, f64, f64, f64) {
    let t7322 = t7140 + t7196 + t7249 + t7320;
    let t7324 = t952 * t1959;
    let t7329 = t2728 * t841;
    let t7332 = t977 * t2208;
    let t7336 = t1402 * t2576;
    (t7322, t7324, t7329, t7332, t7336)
}
