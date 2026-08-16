//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1153/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1153(t1114: f64, t15917: f64, t3931: f64, t4232: f64, t1125: f64, t12550: f64, t15882: f64, t15886: f64, t15891: f64, t15895: f64, t15899: f64, t15902: f64, t15906: f64, t15910: f64, t15914: f64, t3052: f64, t3067: f64, t3080: f64, t9556: f64, t9618: f64) -> f64 {
    let t15918 = t15917 * t1114;
    let t15919 = t3931 * t15918;
    let t15923 = t15917 * t4232;
    let t15924 = t3931 * t15923;
    let t15927 = 5.0_f64 / 13824.0_f64 * t1125 * t15882 - 5.0_f64 / 5184.0_f64 * t1125 * t15886 + t9618 * t15891 / 512.0_f64 - t3067 * t15895 / 2304.0_f64 - t15899 / 3456.0_f64 - t3067 * t15902 / 4608.0_f64 - t9556 * t15906 / 2304.0_f64 - t3067 * t15910 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t3067 * t15914 - t3080 * t15919 / 3072.0_f64 + t12550 / 81.0_f64 + t3052 * t15924 / 1536.0_f64;
    t15927
}
