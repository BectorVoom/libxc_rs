//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1143/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1143(t2857: f64, t412: f64, t11378: f64, t7573: f64, t4524: f64, t7692: f64, t2880: f64, t4544: f64, t7768: f64, t1117: f64, t1123: f64, t1129: f64, t1134: f64, t11379: f64, t11383: f64, t11386: f64, t11392: f64, t3739: f64, t3747: f64, t3788: f64, t4521: f64, t4550: f64, t4553: f64, t4556: f64, t4568: f64, t510: f64, t518: f64, t7806: f64, t7817: f64, t9549: f64, t9587: f64, t9594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11406 = t2857 * t412;
    let t11407 = t11406 * t11378;
    let t11410 = t7573 * t412;
    let t11411 = t11410 * t11378;
    let t11421 = t7692 * t4524;
    let t11430 = t2880 * t4544;
    let t11437 = t7768 * t4524;
    let t11444 = 32.0_f64 * t7806 * t11386 + 200.0_f64 * t9549 * t11383 - 200.0_f64 * t9549 * t11392 - 512.0_f64 / 729.0_f64 * t9594 * t11407 - 128.0_f64 / 81.0_f64 * t3739 * t11411 - 64.0_f64 / 27.0_f64 * t3747 * t11379 - 512.0_f64 / 729.0_f64 * t9587 * t11407 + 252.0_f64 * t1134 * t4556 * t1123 - 336.0_f64 * t518 * t11421 * t1129 - 8.0_f64 * t3788 * t4521 - 4.0_f64 * t1117 * t4568 * t1123 + 6.0_f64 * t510 * t11430 * t1129 + 12.0_f64 * t1117 * t4550 * t1123 - 24.0_f64 * t510 * t11437 * t1129 + 120.0_f64 * t7817 * t4553 * t1123;
    (t11406, t11410, t11411, t11421, t11430, t11437, t11444)
}
