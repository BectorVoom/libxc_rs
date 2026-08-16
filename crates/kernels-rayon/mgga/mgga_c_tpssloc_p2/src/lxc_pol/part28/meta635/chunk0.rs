//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2011/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2011(t90642: f64, t90645: f64, t90659: f64, t90663: f64, t90686: f64, t90701: f64, t12021: f64, t12033: f64, t1375: f64, t16460: f64, t16475: f64, t2092: f64, t27062: f64, t27115: f64, t3758: f64, t3882: f64, t3887: f64, t3888: f64, t3911: f64, t55134: f64, t7194: f64, t7199: f64, t7925: f64, t7936: f64, t81264: f64, t81267: f64, t84423: f64, t90639: f64, t90690: f64, t90704: f64) -> f64 {
    let t93438 = 0.16449340668482264365e-1_f64 * t90642;
    let t93439 = 0.16449340668482264365e-1_f64 * t90645;
    let t93445 = 0.12793931631041761173e0_f64 * t90659;
    let t93446 = 0.16449340668482264365e-1_f64 * t90663;
    let t93452 = 0.3289868133696452873e-1_f64 * t90686;
    let t93461 = 0.16449340668482264365e-1_f64 * t90701;
    let t93465 = -0.3289868133696452873e-1_f64 * t90639 - 6.0_f64 * t7194 * t16475 + t93438 + t93439 + 0.10417915756705434098e0_f64 * t81264 + 4.0_f64 * t3882 * t27062 - 2.0_f64 * t3758 * t27115 - t93445 - t93446 + 2.0_f64 * t1375 * t3887 * t7936 * t3911 + 0.3289868133696452873e-1_f64 * t81267 - t93452 - 0.16449340668482264365e-1_f64 * t90690 + t84423 + 4.0_f64 * t16460 * t7199 - 6.0_f64 * t1375 * t12021 * t7936 * t3888 - t55134 * t2092 + t93461 - 0.16449340668482264365e-1_f64 * t90704 + 2.0_f64 * t12033 * t7925;
    t93465
}
