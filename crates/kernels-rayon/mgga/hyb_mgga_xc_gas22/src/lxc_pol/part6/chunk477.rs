//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 477/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk477(t779: f64, t238: f64, t242: f64, t2176: f64, t226: f64, t2167: f64, t2178: f64, t2196: f64, t2201: f64, t2203: f64, t2207: f64, t2209: f64, t2216: f64, t2218: f64) -> (f64, f64, f64, f64, f64) {
    let t2220 = t779 * t779;
    let t2222 = t238 * t242 * t2220;
    let t2224 = t226 * t2176;
    let t2226 = t238 * t242 * t2224;
    let t2228 = -0.9494625e0_f64 * t2196 + 0.1898925e1_f64 * t2201 + t2203 - 0.59793333333333333334e0_f64 * t2167 + 0.8969e0_f64 * t2178 + 0.15358125e0_f64 * t2207 + 0.3071625e0_f64 * t2209 + t2216 - 0.32862666666666666666e0_f64 * t2218 + 0.24647e0_f64 * t2222 + 0.24647e0_f64 * t2226;
    (t2220, t2222, t2224, t2226, t2228)
}
