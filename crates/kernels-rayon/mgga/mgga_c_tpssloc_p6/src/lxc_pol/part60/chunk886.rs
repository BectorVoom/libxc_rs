//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 886/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk886(t1484: f64, t31376: f64, t6637: f64, t6552: f64, t232: f64, t26656: f64, t6646: f64, t1888: f64, t1894: f64, t7823: f64, t214: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33375 = t31376 * t1484;
    let t33376 = t6637 * t33375;
    let t33377 = t6552 * t33376;
    let t33379 = t26656 * t232;
    let t33380 = t6646 * t33379;
    let t33381 = t1888 * t33380;
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    (t33375, t33376, t33377, t33379, t33380, t33381, t33383, t33384, t33385)
}
