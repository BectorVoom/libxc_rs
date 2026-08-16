//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1392/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1392(t10214: f64, t1041: f64, t13995: f64, t14172: f64, t1539: f64, t1616: f64, t21134: f64, t21566: f64, t21570: f64, t21574: f64, t21595: f64, t2979: f64, t3070: f64, t3071: f64, t43253: f64, t43307: f64, t4582: f64, t50425: f64, t62832: f64, t70846: f64, t70867: f64, t70912: f64, t70929: f64, t76585: f64, t76593: f64, t76608: f64, t76624: f64, t77606: f64, t973: f64, t977: f64) -> f64 {
    let t77761 = t13995 * t21574 / 384.0_f64 + 5.0_f64 / 1152.0_f64 * t13995 * t21570 + t3070 * t3071 * t21134 * t1616 / 1152.0_f64 - t43253 - t973 * t2979 * t76593 / 6.0_f64 - t973 * t977 * t76624 / 36.0_f64 + t973 * t2979 * t76608 / 54.0_f64 + t70846 / 576.0_f64 - t70867 / 36.0_f64 - t43307 - t62832 / 162.0_f64 - 5.0_f64 / 384.0_f64 * t1041 * t4582 * t14172 * t77606 + 7.0_f64 / 108.0_f64 * t973 * t10214 * t76585 + 5.0_f64 / 1728.0_f64 * t70912 + 5.0_f64 / 972.0_f64 * t50425 + t3070 * t3071 * t21595 * t1539 / 1152.0_f64 + t13995 * t21566 / 384.0_f64 + t70929 / 54.0_f64;
    t77761
}
