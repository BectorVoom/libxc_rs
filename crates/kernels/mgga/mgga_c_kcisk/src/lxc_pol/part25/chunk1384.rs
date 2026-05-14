//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1384/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1384<F: Float>(t33162: F, t34573: F, t34484: F, t9733: F, t12254: F, t2029: F, t10008: F, t10879: F, t9740: F, t10831: F, t1310: F, t786: F, t10463: F, t10005: F, t112665: F, t112765: F, t113152: F, t113155: F, t117167: F, t117868: F, t15921: F, t15930: F, t33193: F, t34419: F, t34422: F, t34548: F, t34560: F, t34561: F, t7261: F, t74445: F, t74475: F) -> (F,) {
    let t118450 = t34573 * t33162;
    let t118455 = 0.34722222222222222222e-2 * t9733 * t34484;
    let t118460 = t12254 * t2029;
    let t118466 = t9740 * t10879 * t10008;
    let t118473 = t1310 * t10831 * t786;
    let t118474 = t2029 * t10463;
    let t118487 = -0.35740740740740740742e-2 * t118450 - 0.13888888888888888889e-1 * t10005 * t33193 + t118455 - 0.20833333333333333334e-1 * t9740 * t7261 * t34422 * t74445 + 0.31250000000000000001e-1 * t9740 * t7261 * t118460 * t74475 + 0.38580246913580246913e-3 * t118466 - 0.23148148148148148148e-2 * t9740 * t34560 * t34561 * t15921 - 0.54012345679012345679e-2 * t9740 * t118473 * t118474 * t15930 + 0.23280625e-2 * t34419 * t117868 + 0.13402777777777777778e-2 * t112765 * t34548 + 0.11607361111111111111e-2 * t112665 + 0.34822083333333333332e-2 * t117167 - 0.5787037037037037037e-3 * t113152 - 0.77160493827160493826e-3 * t113155;
    (t118487,)
}
