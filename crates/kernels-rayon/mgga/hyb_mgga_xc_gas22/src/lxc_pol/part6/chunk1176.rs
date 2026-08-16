//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1176/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1176(t7108: f64, t993: f64, t21389: f64, t7058: f64, t974: f64, t2536: f64, t2558: f64, t365: f64, t21424: f64, t2561: f64, t2530: f64, t2537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21616 = t993 * t7108;
    let t21620 = 0.17757530864197530864e0_f64 * t21389;
    let t21628 = t974 * t7058;
    let t21633 = t365 / t2558 / t2536;
    let t21638 = 0.5356037037037037037e1_f64 * t21389;
    let t21641 = 0.16979925925925925926e1_f64 * t21424;
    let t21668 = t2558 * t2558;
    let t21670 = t365 / t21668;
    let t21671 = t2561 * t2561;
    let t21672 = 1.0_f64 / t21671;
    let t21676 = t2530 * t2537;
    (t21616, t21620, t21628, t21633, t21638, t21641, t21670, t21672, t21676)
}
