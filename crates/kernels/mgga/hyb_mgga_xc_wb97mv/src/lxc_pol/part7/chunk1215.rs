//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1215/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1215<F: Float>(t10324: F, t1852: F, t10333: F, t8225: F, t25: F, t2707: F, t33: F, t34: F, t3890: F, t6134: F, t3886: F, t3882: F, t10360: F, t10322: F, t1173: F, t1857: F, t1860: F, t1861: F, t1868: F, t1877: F, t21354: F, t24947: F, t24962: F, t29290: F, t29291: F, t3040: F, t3864: F, t453: F, t571: F, t574: F, t8228: F) -> (F, F) {
    let t29364 = t1852 * t10324;
    let t29366 = t8225 * t10333;
    let t29383 = t33 * t34 / t25 / t2707;
    let t29392 = t6134 * t3890;
    let t29398 = t6134 * t3886;
    let t29404 = t6134 * t3882;
    let t29406 = t1852 * t10360;
    let t29408 = 10.0 / 729.0 * t29364 + 44.0 / 243.0 * t29366 - 5.0 / 243.0 * t571 * t8228 * t10322 * t1877 - 40.0 / 729.0 * t571 * t24947 * t21354 * t3864 * t1861 - 8.0 / 9.0 * t29290 * t24962 * t29291 - 8.0 / 81.0 * t29383 * t1857 * t1860 * t453 + 8.0 / 27.0 * t29383 * t574 * t1868 * t453 - 2.0 / 243.0 * t29392 + 4.0 / 27.0 * t3040 * t574 * t1868 * t1173 + 4.0 / 243.0 * t29398 - 4.0 / 81.0 * t3040 * t1857 * t1860 * t1173 - 4.0 / 729.0 * t29404 + t29406 / 81.0;
    (t29383, t29408)
}
