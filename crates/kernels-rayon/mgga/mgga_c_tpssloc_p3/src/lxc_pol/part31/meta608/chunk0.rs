//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1853/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1853(t90791: f64, t90794: f64, t90797: f64, t90805: f64, t90844: f64, t90859: f64, t90864: f64, t90866: f64, t90898: f64, t90912: f64, t90956: f64, t90961: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93489 = 0.15352717957250113407e0_f64 * t90791;
    let t93490 = 0.3289868133696452873e-1_f64 * t90794;
    let t93491 = 0.3289868133696452873e-1_f64 * t90797;
    let t93494 = 0.3289868133696452873e-1_f64 * t90805;
    let t93524 = 0.3289868133696452873e-1_f64 * t90844;
    let t93528 = 0.16449340668482264365e-1_f64 * t90859;
    let t93529 = 0.16449340668482264365e-1_f64 * t90864;
    let t93537 = 0.76763589786250567036e-1_f64 * t90866;
    let t93562 = 0.3289868133696452873e-1_f64 * t90898;
    let t93572 = 0.15352717957250113407e0_f64 * t90912;
    let t93588 = 0.76763589786250567036e-1_f64 * t90956;
    let t93589 = 0.3289868133696452873e-1_f64 * t90961;
    (t93489, t93490, t93491, t93494, t93524, t93528, t93529, t93537, t93562, t93572, t93588, t93589)
}
