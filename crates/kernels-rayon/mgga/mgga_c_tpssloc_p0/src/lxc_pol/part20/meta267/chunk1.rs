//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1424/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1424(t2960: f64, t2974: f64, t3014: f64, t984: f64, t340: f64, t343: f64, t974: f64, t135: f64, t3016: f64, t973: f64, t10263: f64, t10267: f64, t10274: f64, t10280: f64, t10283: f64, t10287: f64, t10290: f64, t10328: f64, t10331: f64, t10333: f64, t10339: f64, t2996: f64, t3000: f64, t3011: f64, t3017: f64, t346: f64, t987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10342 = t2960 * t2974;
    let t10346 = t3014 * t984;
    let t10348 = t340 * t10346 * t343;
    let t10349 = t974 * t10348;
    let t10352 = t135 * t3016;
    let t10353 = t973 * t10352;
    let t10357 = -0.14814814814814814814e-2_f64 * t10267 - 0.22222222222222222221e-2_f64 * t2960 * t3000 + 0.44444444444444444442e-2_f64 * t2960 * t2996 - 0.55555555555555555554e-3_f64 * t10274 - 0.22222222222222222221e-2_f64 * t973 * t10280 - 0.38024691358024691358e-1_f64 * t10283 * t346 + 0.55555555555555555554e-3_f64 * t10287 - 0.83333333333333333331e-3_f64 * t10290 - 0.83333333333333333332e-3_f64 * t973 * t10328 + 0.81481481481481481478e-2_f64 * t10331 + 0.14814814814814814814e-2_f64 * t10333 + t10339 - 0.24444444444444444444e-1_f64 * t10263 * t987 + 0.44444444444444444443e-2_f64 * t10342 + 0.66666666666666666666e-2_f64 * t2960 * t3011 - 0.83333333333333333332e-3_f64 * t973 * t10349 - 0.83333333333333333331e-3_f64 * t10353 + 0.66666666666666666666e-2_f64 * t2960 * t3017;
    (t10342, t10346, t10348, t10352, t10353, t10357)
}
