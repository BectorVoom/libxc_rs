//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2699/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2699(t16336: f64, t5314: f64, t1831: f64, t53880: f64, t19930: f64, t3866: f64, t1351: f64, t5187: f64, t6414: f64, t120: f64, t19731: f64, t12336: f64, t12429: f64, t1363: f64, t1367: f64, t16227: f64, t16248: f64, t16305: f64, t16311: f64, t16321: f64, t16394: f64, t19871: f64, t19958: f64, t3783: f64, t3793: f64, t3803: f64, t3807: f64, t5246: f64, t5248: f64, t5250: f64, t53910: f64, t54047: f64, t54059: f64, t56275: f64, t6427: f64, t6431: f64, t820: f64) -> (f64, f64) {
    let t56779 = t16336 * t5314;
    let t56795 = t53880 * t1831;
    let t56797 = t3866 * t19930;
    let t56805 = t5187 * t1351;
    let t56812 = t6414 * t1351;
    let t56817 = t120 * t19731;
    let t56826 = 7.0_f64 / 288.0_f64 * t56779 - t53910 * t1831 / 384.0_f64 - t16321 * t5314 / 192.0_f64 + 5.0_f64 / 768.0_f64 * t12336 * t6427 - t12336 * t6431 / 768.0_f64 - t3783 * t19930 / 384.0_f64 - t1363 * t1367 * t820 * t56275 / 768.0_f64 - 119.0_f64 / 1728.0_f64 * t56795 + 7.0_f64 / 576.0_f64 * t56797 - 119.0_f64 / 3456.0_f64 * t54047 + 35.0_f64 / 576.0_f64 * t54059 + 7.0_f64 / 1536.0_f64 * t5246 * t5248 * t19871 * t3793 - t5246 * t16305 * t16311 * t56805 / 96.0_f64 - 5.0_f64 / 192.0_f64 * t16394 * t16227 + t3803 * t16305 * t56812 * t3807 / 384.0_f64 + t5246 * t5248 * t56817 * t5250 / 768.0_f64 + t16394 * t16248 / 384.0_f64 + t12429 * t19958 / 384.0_f64;
    (t56817, t56826)
}
