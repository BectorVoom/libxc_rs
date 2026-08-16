//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 461/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk461(t7: f64, t143: f64, t172: f64, t187: f64, t2103: f64, t2104: f64, t2147: f64, t740: f64, t759: f64, t139: f64, t214: f64, t26: f64, t1796: f64, t1885: f64, t222: f64, t226: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t144 = 0.135e1_f64 <= t143;
    let t2151 = piecewise3(t144, t2103, -8.0_f64 / 3.0_f64 * t2104 * t187 - 16.0_f64 / 3.0_f64 * t740 * t759 - 8.0_f64 / 3.0_f64 * t172 * t2147);
    let t2152 = t139 * t2151;
    let t2153 = t2152 * t214;
    let t2154 = t26 * t2153;
    let t2159 = piecewise3(t8, 0.0_f64, t1796);
    let t2164 = t222 * t1885 * t226;
    (t2151, t2152, t2153, t2154, t2159, t2164)
}
