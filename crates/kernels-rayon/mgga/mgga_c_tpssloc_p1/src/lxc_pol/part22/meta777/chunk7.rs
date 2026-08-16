//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2662/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662(t20512: f64, t40021: f64, t1351: f64, t6347: f64, t16288: f64, t6422: f64, t1363: f64, t1367: f64, t16225: f64, t16233: f64, t16305: f64, t16311: f64, t1827: f64, t19855: f64, t19904: f64, t20473: f64, t5246: f64, t5289: f64, t5310: f64, t53985: f64, t53998: f64, t56693: f64, t56710: f64, t56738: f64, t56924: f64, t57342: f64, t74355: f64, t820: f64) -> f64 {
    let t74360 = t40021 * t20512;
    let t74366 = t6347 * t1351;
    let t74376 = t16288 * t6422;
    let t74386 = -t1363 * t1367 * t820 * t74355 / 768.0_f64 + 7.0_f64 / 12.0_f64 * t74360 - t5246 * t16305 * t20473 * t16225 / 128.0_f64 - t5246 * t16305 * t16311 * t74366 / 128.0_f64 + 3.0_f64 / 128.0_f64 * t16233 * t16305 * t57342 * t16225 - 7.0_f64 / 768.0_f64 * t56693 + 7.0_f64 / 1536.0_f64 * t74376 - 7.0_f64 / 192.0_f64 * t56710 - t53985 + t53998 + 7.0_f64 / 48.0_f64 * t56738 - t56924 * t1827 / 1024.0_f64 - t19855 * t5289 / 1024.0_f64 + 5.0_f64 / 256.0_f64 * t19904 * t5310;
    t74386
}
