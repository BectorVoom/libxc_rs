//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3006/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006(t10422: f64, t18020: f64, t3070: f64, t10883: f64, t13969: f64, t17979: f64, t17620: f64, t2960: f64, t10390: f64, t17649: f64, t17980: f64, t17984: f64, t3146: f64, t42565: f64, t43211: f64, t43307: f64, t43325: f64, t43336: f64, t43341: f64, t50343: f64, t50361: f64, t50378: f64, t50384: f64, t55723: f64, t973: f64, t974: f64) -> f64 {
    let t62811 = t3070 * t10422 * t18020;
    let t62816 = t10883 * t13969 * t17979;
    let t62827 = t2960 * t17620;
    let t62829 = 5.0_f64 / 10368.0_f64 * t50343 - t50361 / 324.0_f64 - t43307 + t50378 / 1728.0_f64 - t10390 * t17649 / 1152.0_f64 - t50384 / 324.0_f64 + t62811 / 3456.0_f64 + t42565 * t17984 / 48.0_f64 + t62816 / 2304.0_f64 - t43211 * t17980 / 288.0_f64 + t973 * t974 * t3146 * t55723 / 108.0_f64 + t43325 / 243.0_f64 + t43336 / 10368.0_f64 - 5.0_f64 / 62208.0_f64 * t43341 - 2.0_f64 / 243.0_f64 * t62827;
    t62829
}
