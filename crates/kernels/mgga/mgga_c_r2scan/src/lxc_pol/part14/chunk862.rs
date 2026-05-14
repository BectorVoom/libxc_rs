//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 862/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk862<F: Float>(t10810: F, t2150: F, t574: F, t3308: F, t6402: F, t1266: F, t507: F, t512: F, t3332: F, t6536: F, t6535: F, t6541: F, t2147: F, t6166: F, t6165: F, t3333: F, t6395: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10811 = t10810 * t2150;
    let t10812 = t574 * t10811;
    let t10813 = 0.23115257973478049502e0 * t10812;
    let t10814 = t3308 * t6402;
    let t10815 = t574 * t10814;
    let t10818 = t512 * t1266 * t507;
    let t10820 = t3332 * t6536;
    let t10821 = t6535 * t10820;
    let t10823 = t3332 * t6541;
    let t10824 = t2147 * t10823;
    let t10826 = t3332 * t6166;
    let t10827 = t6165 * t10826;
    let t10829 = t6395 * t3333;
    (t10811, t10812, t10813, t10814, t10815, t10818, t10820, t10821, t10823, t10824, t10826, t10827, t10829)
}
