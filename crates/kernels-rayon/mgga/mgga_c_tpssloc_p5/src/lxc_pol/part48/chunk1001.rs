//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1001/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1001(t22724: f64, t31569: f64, t1985: f64, t214: f64, t225: f64, t24063: f64, t567: f64, t31589: f64, t6897: f64, t794: f64, t114297: f64, t114300: f64, t114317: f64, t12021: f64, t12030: f64, t12444: f64, t1323: f64, t1375: f64, t2016: f64, t22656: f64, t24082: f64, t24147: f64, t31555: f64, t31564: f64, t31584: f64, t3758: f64, t3882: f64, t3887: f64, t3888: f64, t568: f64, t6958: f64, t6963: f64, t6992: f64, t7199: f64, t7213: f64, t84433: f64, t8627: f64, t8636: f64) -> f64 {
    let t115629 = t22724 * t31569;
    let t115630 = 0.26044789391763585244e-1_f64 * t115629;
    let t115638 = t1985 * t214 * t24063 * t225 * t567;
    let t115658 = t6897 * t794 * t31589;
    let t115660 = t114297 + 4.0_f64 * t6958 * t24147 - t114300 + 4.0_f64 * t24082 * t6963 + 4.0_f64 * t3758 * t31564 + t115630 - 6.0_f64 * t1375 * t12021 * t8636 * t3888 + 0.82246703342411321825e-2_f64 * t115638 + 4.0_f64 * t3882 * t31555 + t114317 - 2.0_f64 * t84433 * t2016 + 2.0_f64 * t1323 * t31584 * t568 + 2.0_f64 * t12030 * t8627 + 4.0_f64 * t12444 * t8627 + 4.0_f64 * t22656 * t7199 + 4.0_f64 * t1375 * t3887 * t7213 * t6992 - 0.82246703342411321824e-2_f64 * t115658;
    t115660
}
