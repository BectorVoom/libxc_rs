//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1153/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1153<F: Float>(t1114: F, t15917: F, t3931: F, t4232: F, t1125: F, t12550: F, t15882: F, t15886: F, t15891: F, t15895: F, t15899: F, t15902: F, t15906: F, t15910: F, t15914: F, t3052: F, t3067: F, t3080: F, t9556: F, t9618: F) -> F {
    let t15918 = t15917 * t1114;
    let t15919 = t3931 * t15918;
    let t15923 = t15917 * t4232;
    let t15924 = t3931 * t15923;
    let t15927 = F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1125 * t15882 - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1125 * t15886 + t9618 * t15891 / F::cast_from(512.0_f64) - t3067 * t15895 / F::cast_from(2304.0_f64) - t15899 / F::cast_from(3456.0_f64) - t3067 * t15902 / F::cast_from(4608.0_f64) - t9556 * t15906 / F::cast_from(2304.0_f64) - t3067 * t15910 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3067 * t15914 - t3080 * t15919 / F::cast_from(3072.0_f64) + t12550 / F::cast_from(81.0_f64) + t3052 * t15924 / F::cast_from(1536.0_f64);
    t15927
}
