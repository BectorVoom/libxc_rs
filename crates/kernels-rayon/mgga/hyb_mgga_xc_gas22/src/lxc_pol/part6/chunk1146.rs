//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1146/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1146(t11478: f64, t3668: f64, t3672: f64, t11470: f64, t3740: f64, t1117: f64, t1123: f64, t1129: f64, t1134: f64, t11447: f64, t11454: f64, t11461: f64, t11466: f64, t11471: f64, t11474: f64, t11475: f64, t2903: f64, t3739: f64, t3747: f64, t3757: f64, t4559: f64, t4562: f64, t518: f64, t7806: f64, t7811: f64, t9527: f64, t9538: f64) -> (f64, f64, f64, f64) {
    let t11479 = t11478 * t3668;
    let t11482 = t11478 * t3672;
    let t11485 = t3740 * t11470;
    let t11495 = -180.0_f64 * t2903 * t11447 * t1129 + 30.0_f64 * t2903 * t4559 * t1123 - 36.0_f64 * t1134 * t11454 * t1129 - 36.0_f64 * t1134 * t4562 * t1123 + 42.0_f64 * t518 * t11461 * t1129 - 4.0_f64 * t1117 * t11466 + 176.0_f64 / 81.0_f64 * t3757 * t11471 + 32.0_f64 / 9.0_f64 * t7811 * t11475 + 32.0_f64 / 9.0_f64 * t7811 * t11479 - 16.0_f64 / 3.0_f64 * t9527 * t11482 + 352.0_f64 / 243.0_f64 * t3739 * t11485 + 176.0_f64 / 81.0_f64 * t3747 * t11471 - 80.0_f64 / 3.0_f64 * t9538 * t11474 * t3668 + 32.0_f64 * t7806 * t11475;
    (t11479, t11482, t11485, t11495)
}
