//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1166/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1166<F: Float>(t11724: F, t26278: F, t11675: F, t26282: F, t12550: F, t2207: F, t3328: F, t1058: F, t9418: F, t11780: F, t3606: F, t10760: F, t22820: F, t29279: F) -> (F, F, F, F, F, F) {
    let t43183 = t26278 * t11724;
    let t43185 = t26282 * t11675;
    let t43188 = t2207 * t12550 * t3328;
    let t43191 = t2207 * t1058 * t9418;
    let t43195 = t2207 * t11780 * t3606;
    let t43200 = t22820 * t10760 * t29279;
    (t43183, t43185, t43188, t43191, t43195, t43200)
}
