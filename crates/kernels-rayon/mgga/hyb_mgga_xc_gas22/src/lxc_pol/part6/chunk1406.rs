//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1406/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1406(t2754: f64, t4482: f64, t2757: f64, t1057: f64, t11237: f64, t1052: f64, t11235: f64, t2751: f64, t21975: f64, t21978: f64, t21982: f64, t21984: f64, t25973: f64, t25975: f64, t25977: f64, t25980: f64, t25982: f64, t25984: f64, t25986: f64, t25990: f64) -> f64 {
    let t30410 = t2754 * t4482;
    let t30412 = t2757 * t4482;
    let t30414 = t1057 * t11237;
    let t30416 = t1052 * t11235;
    let t30418 = t1057 * t11235;
    let t30422 = t2751 * t4482;
    let t30431 = 12.0_f64 * t30410 - t21975 - 32.0_f64 * t30412 - 8.0_f64 * t30414 + 8.0_f64 * t30416 - 8.0_f64 * t30418 - 32.0_f64 * t25973 - 8.0_f64 * t25975 + 20.0_f64 * t30422 - 8.0_f64 * t25977 + 8.0_f64 * t25980 - 48.0_f64 * t25982 - 48.0_f64 * t25984 + 96.0_f64 * t25986 - 8.0_f64 * t21978 - t21982 + t21984 + 160.0_f64 * t25990;
    t30431
}
