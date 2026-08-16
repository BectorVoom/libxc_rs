//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1206/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1206(t1484: f64, t30622: f64, t23270: f64, t22986: f64, t30676: f64, t6637: f64, t6552: f64, t232: f64, t25261: f64, t6646: f64, t1888: f64, t1894: f64, t7510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32814 = t30622 * t1484;
    let t32815 = t23270 * t32814;
    let t32817 = 0.3289868133696452873e-1_f64 * t22986 * t32815;
    let t32818 = t30676 * t1484;
    let t32819 = t6637 * t32818;
    let t32821 = 0.3289868133696452873e-1_f64 * t6552 * t32819;
    let t32822 = t25261 * t232;
    let t32823 = t6646 * t32822;
    let t32825 = 0.16449340668482264365e-1_f64 * t1888 * t32823;
    let t32826 = t1894 * t7510;
    (t32814, t32815, t32817, t32818, t32819, t32821, t32822, t32823, t32825, t32826)
}
