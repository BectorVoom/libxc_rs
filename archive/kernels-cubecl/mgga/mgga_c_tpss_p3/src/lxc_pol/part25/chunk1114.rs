//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1114/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1114<F: Float>(t1025: F, t15291: F, t1032: F, t5104: F, t673: F, t5085: F, t9271: F, t1027: F, t4079: F, t4087: F, t2885: F, t5092: F) -> (F, F, F, F, F, F) {
    let t15292 = t1025 * t15291;
    let t15294 = t1032 * t15291;
    let t15296 = t673 * t5104;
    let t15298 = t9271 * t5085;
    let t15299 = t15298 * t1027;
    let t15301 = t4087 * t4079;
    let t15303 = t2885 * t5092;
    (t15292, t15294, t15296, t15299, t15301, t15303)
}
