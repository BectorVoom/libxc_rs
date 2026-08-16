//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2703/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703(t1340: f64, t56923: f64, t12365: f64, t6417: f64, t12283: f64, t19962: f64, t19882: f64, t19996: f64, t3866: f64, t40018: f64, t6371: f64, t119: f64, t12351: f64, t12419: f64, t12420: f64, t1343: f64, t1354: f64, t1363: f64, t16321: f64, t19871: f64, t210: f64, t3733: f64, t3734: f64, t3790: f64, t3803: f64, t5310: f64, t54151: f64, t54191: f64, t54198: f64, t56486: f64, t56906: f64, t56909: f64, t56914: f64, t56919: f64, t56921: f64, t6347: f64, t820: f64) -> f64 {
    let t56924 = t56923 * t1340;
    let t56927 = t12365 * t6417;
    let t56933 = t12283 * t19962;
    let t56935 = t12283 * t19882;
    let t56937 = t3866 * t19996;
    let t56946 = t40018 * t6371;
    let t56952 = -7.0_f64 / 288.0_f64 * t56906 + 595.0_f64 / 5184.0_f64 * t54151 - 35.0_f64 / 288.0_f64 * t56909 + 5.0_f64 / 192.0_f64 * t16321 * t5310 + t3790 * t1343 * t820 * t56914 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t56919 + 7.0_f64 / 1152.0_f64 * t56921 - t56924 * t1354 / 1536.0_f64 - 119.0_f64 / 13824.0_f64 * t56927 - 5.0_f64 / 768.0_f64 * t3803 * t12419 * t19871 * t12420 + 7.0_f64 / 2304.0_f64 * t56933 - 7.0_f64 / 576.0_f64 * t56935 - 35.0_f64 / 576.0_f64 * t56937 - 5.0_f64 / 128.0_f64 * t1363 * t12351 * t820 * t6347 * t3734 + 35.0_f64 / 96.0_f64 * t54191 - 119.0_f64 / 3456.0_f64 * t54198 + 35.0_f64 / 72.0_f64 * t56946 + t3733 * t210 * t119 * t56486 / 8.0_f64;
    t56952
}
