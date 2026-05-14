//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1161/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1161<F: Float>(t1748: F, t5234: F, t1751: F, t5938: F, t1398: F, t1745: F, t735: F, t5219: F, t5943: F, t1668: F, t1757: F, t1871: F, t615: F, t1684: F, t5402: F, t591: F, t5946: F) -> (F, F, F, F, F, F) {
    let t21177 = t5234 * t1748;
    let t21179 = t1751 * t5938;
    let t21183 = 0.43374325201206959368e-1 * t735 * t1398 * t1745;
    let t21186 = t5219 * t5943;
    let t21191 = 0.13549023786666666666e-1 * t1757 * t615 * t1668 * t1871;
    let t21195 = 0.13549023786666666666e-1 * t5946 * t1684 * t5402 * t591;
    (t21177, t21179, t21183, t21186, t21191, t21195)
}
