//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1149/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1149<F: Float>(t15842: F, t3931: F, t461: F, t5248: F, t4232: F, t1015: F, t5254: F, t3068: F, t1125: F, t12435: F, t12446: F, t12448: F, t12472: F, t15828: F, t15832: F, t15835: F, t15839: F, t3052: F, t3080: F, t4253: F, t4271: F, t9573: F, t9626: F) -> (F, F) {
    let t15843 = t3931 * t15842;
    let t15846 = t461 * t5248;
    let t15847 = t15846 * t4232;
    let t15848 = t3931 * t15847;
    let t15854 = t5254 * t1015;
    let t15855 = t3068 * t15854;
    let t15860 = -t1125 * t15828 / F::cast_from(1152.0_f64) - t12446 / F::cast_from(6912.0_f64) + t15832 / F::cast_from(162.0_f64) - t15835 / F::cast_from(864.0_f64) + t3052 * t15839 / F::cast_from(768.0_f64) - t3080 * t15843 / F::cast_from(1536.0_f64) - t9626 * t15848 / F::cast_from(512.0_f64) + t12448 / F::cast_from(1296.0_f64) + t12435 * t4253 / F::cast_from(288.0_f64) + t9573 * t15855 / F::cast_from(4608.0_f64) + t12472 * t4271 / F::cast_from(432.0_f64);
    (t15846, t15860)
}
