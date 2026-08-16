//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 944/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk944<F: Float>(t10823: F, t2147: F, t3332: F, t6166: F, t6165: F, t3333: F, t6395: F, t1266: F, t260: F, t259: F, t277: F, t254: F) -> (F, F, F, F, F, F, F) {
    let t10824 = t2147 * t10823;
    let t10826 = t3332 * t6166;
    let t10827 = t6165 * t10826;
    let t10829 = t6395 * t3333;
    let t10831 = t260 * t1266;
    let t10833 = t259 * t10831 * t277;
    let t10834 = t254 * t10833;
    (t10824, t10826, t10827, t10829, t10831, t10833, t10834)
}
