//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1438/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1438(t2869: f64, t4501: f64, t1572: f64, t1849: f64, t1620: f64, t3972: f64, t1145: f64, t11478: f64, t14635: f64, t14638: f64, t14641: f64, t14648: f64, t14770: f64, t14818: f64, t22746: f64, t22750: f64, t26560: f64, t2851: f64, t2858: f64, t2893: f64, t2923: f64, t30767: f64, t30915: f64, t30956: f64, t3739: f64, t4530: f64, t4576: f64, t7721: f64, t7800: f64, t9493: f64) -> (f64, f64, f64) {
    let t31237 = t4501 * t2869;
    let t31246 = t1849 * t1572;
    let t31247 = t3972 * t1620;
    let t31248 = t31246 * t31247;
    let t31271 = -320.0_f64 / 27.0_f64 * t14818 * t2851 * t31237 - 448.0_f64 / 27.0_f64 * t14770 * t30956 - 160.0_f64 / 9.0_f64 * t7800 * t2858 * t31237 + 10000.0_f64 / 27.0_f64 * t14635 * t31248 + 50000.0_f64 / 27.0_f64 * t14638 * t31248 - 90.0_f64 * t7721 * t1145 * t4530 * t2893 + 5040.0_f64 * t26560 * t4576 * t2923 + 40000.0_f64 / 9.0_f64 * t14641 * t31248 + 70000.0_f64 / 27.0_f64 * t14648 * t31248 - 224.0_f64 * t22750 * t30767 + 160.0_f64 * t22746 * t11478 * t9493 - 3872.0_f64 / 729.0_f64 * t3739 * t30915;
    (t31247, t31248, t31271)
}
