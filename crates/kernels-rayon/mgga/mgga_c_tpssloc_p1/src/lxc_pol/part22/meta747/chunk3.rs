//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2491/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2491(t21126: f64, t2970: f64, t973: f64, t1023: f64, t1031: f64, t13995: f64, t17677: f64, t21130: f64, t21482: f64, t21490: f64, t21493: f64, t2960: f64, t3070: f64, t378: f64, t42397: f64, t43307: f64, t4579: f64, t50362: f64, t61950: f64, t62811: f64, t62816: f64) -> f64 {
    let t70867 = t973 * t2970 * t21126;
    let t70884 = -t50362 + t2960 * t21490 / 18.0_f64 - t70867 / 144.0_f64 - t2960 * t21493 / 27.0_f64 - t43307 - t21482 * t1031 * t378 / 576.0_f64 + t13995 * t17677 / 768.0_f64 + t62811 / 2304.0_f64 + 5.0_f64 / 5184.0_f64 * t3070 * t42397 * t21130 * t1023 + t61950 * t4579 / 1536.0_f64 + t62816 / 1536.0_f64;
    t70884
}
