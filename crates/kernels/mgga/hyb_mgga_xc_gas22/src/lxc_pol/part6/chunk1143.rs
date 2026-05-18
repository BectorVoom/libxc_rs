//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1143/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1143<F: Float>(t2857: F, t412: F, t11378: F, t7573: F, t4524: F, t7692: F, t2880: F, t4544: F, t7768: F, t1117: F, t1123: F, t1129: F, t1134: F, t11379: F, t11383: F, t11386: F, t11392: F, t3739: F, t3747: F, t3788: F, t4521: F, t4550: F, t4553: F, t4556: F, t4568: F, t510: F, t518: F, t7806: F, t7817: F, t9549: F, t9587: F, t9594: F) -> (F, F, F, F, F, F, F) {
    let t11406 = t2857 * t412;
    let t11407 = t11406 * t11378;
    let t11410 = t7573 * t412;
    let t11411 = t11410 * t11378;
    let t11421 = t7692 * t4524;
    let t11430 = t2880 * t4544;
    let t11437 = t7768 * t4524;
    let t11444 = F::new(32.0) * t7806 * t11386 + F::new(200.0) * t9549 * t11383 - F::new(200.0) * t9549 * t11392 - F::new(512.0) / F::new(729.0) * t9594 * t11407 - F::new(128.0) / F::new(81.0) * t3739 * t11411 - F::new(64.0) / F::new(27.0) * t3747 * t11379 - F::new(512.0) / F::new(729.0) * t9587 * t11407 + F::new(252.0) * t1134 * t4556 * t1123 - F::new(336.0) * t518 * t11421 * t1129 - F::new(8.0) * t3788 * t4521 - F::new(4.0) * t1117 * t4568 * t1123 + F::new(6.0) * t510 * t11430 * t1129 + F::new(12.0) * t1117 * t4550 * t1123 - F::new(24.0) * t510 * t11437 * t1129 + F::new(120.0) * t7817 * t4553 * t1123;
    (t11406, t11410, t11411, t11421, t11430, t11437, t11444)
}
