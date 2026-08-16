//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1558/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558(t12772: f64, t24786: f64, t3625: f64, t17572: f64, t21188: f64, t13052: f64, t24667: f64, t3172: f64, t12916: f64, t24705: f64, t3718: f64, t1222: f64, t17240: f64, t24244: f64) -> (f64, f64, f64, f64, f64) {
    let t83435 = t3625 * t12772 * t24786;
    let t83462 = t17572 * t21188;
    let t83485 = t13052 * t3172 * t24667;
    let t83490 = t3718 * t12916 * t24705;
    let t83504 = t1222 * t17240 * t24244;
    (t83435, t83462, t83485, t83490, t83504)
}
