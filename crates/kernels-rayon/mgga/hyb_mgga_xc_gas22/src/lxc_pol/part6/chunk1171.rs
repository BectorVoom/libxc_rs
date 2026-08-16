//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1171/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1171(t2249: f64, t2271: f64, t262: f64, t20624: f64, t20688: f64, t2274: f64, t7147: f64, t944: f64, t2519: f64, t347: f64, t2522: f64, t2470: f64, t2477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21053 = t262 / t2271 / t2249;
    let t21057 = 0.5356037037037037037e1_f64 * t20624;
    let t21071 = 0.16979925925925925926e1_f64 * t20688;
    let t21087 = t2271 * t2271;
    let t21089 = t262 / t21087;
    let t21090 = t2274 * t2274;
    let t21091 = 1.0_f64 / t21090;
    let t21366 = t944 * t7147;
    let t21369 = t2519 * t2519;
    let t21371 = t347 / t21369;
    let t21373 = t2522 * t2522;
    let t21374 = 1.0_f64 / t21373;
    let t21378 = t2470 * t2477;
    (t21053, t21057, t21071, t21089, t21091, t21366, t21371, t21374, t21378)
}
