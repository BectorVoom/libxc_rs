//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1097/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1097<F: Float>(t10769: F, t39409: F, t2547: F, t37764: F, t25397: F, t37945: F, t38031: F, t10710: F, t10768: F, t25737: F, t25499: F, t37586: F) -> (F, F, F, F, F) {
    let t39410 = t39409 * t10769;
    let t39420 = t37764 * t2547;
    let t39429 = t38031 * t37945 * t25397;
    let t39437 = t10768 * t10710 * t25737;
    let t39440 = t37586 * t10710 * t25499;
    (t39410, t39420, t39429, t39437, t39440)
}
