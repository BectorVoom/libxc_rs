//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1146/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1146<F: Float>(t11478: F, t3668: F, t3672: F, t11470: F, t3740: F, t1117: F, t1123: F, t1129: F, t1134: F, t11447: F, t11454: F, t11461: F, t11466: F, t11471: F, t11474: F, t11475: F, t2903: F, t3739: F, t3747: F, t3757: F, t4559: F, t4562: F, t518: F, t7806: F, t7811: F, t9527: F, t9538: F) -> (F, F, F, F) {
    let t11479 = t11478 * t3668;
    let t11482 = t11478 * t3672;
    let t11485 = t3740 * t11470;
    let t11495 = -F::cast_from(180.0_f64) * t2903 * t11447 * t1129 + F::cast_from(30.0_f64) * t2903 * t4559 * t1123 - F::cast_from(36.0_f64) * t1134 * t11454 * t1129 - F::cast_from(36.0_f64) * t1134 * t4562 * t1123 + F::cast_from(42.0_f64) * t518 * t11461 * t1129 - F::cast_from(4.0_f64) * t1117 * t11466 + F::cast_from(176.0_f64) / F::cast_from(81.0_f64) * t3757 * t11471 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t11475 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t11479 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t9527 * t11482 + F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t3739 * t11485 + F::cast_from(176.0_f64) / F::cast_from(81.0_f64) * t3747 * t11471 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t9538 * t11474 * t3668 + F::cast_from(32.0_f64) * t7806 * t11475;
    (t11479, t11482, t11485, t11495)
}
