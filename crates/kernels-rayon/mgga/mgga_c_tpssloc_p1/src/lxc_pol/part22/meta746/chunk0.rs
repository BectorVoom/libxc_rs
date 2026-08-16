//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2481/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481(t18030: f64, t4630: f64, t17884: f64, t4644: f64, t13969: f64, t21502: f64, t3039: f64, t10214: f64, t1041: f64, t14080: f64, t14164: f64, t21603: f64, t2979: f64, t3048: f64, t4582: f64, t47775: f64, t5861: f64, t62282: f64, t62284: f64, t68521: f64, t68534: f64, t68539: f64, t70330: f64, t70339: f64, t973: f64, t977: f64) -> (f64, f64) {
    let t70554 = t18030 * t4630;
    let t70573 = t4644 * t17884;
    let t70597 = t3039 * t13969 * t21502;
    let t70599 = -t3048 * t21603 / 864.0_f64 + 5.0_f64 / 6912.0_f64 * t70573 - t62282 / 216.0_f64 - t62284 / 3456.0_f64 - 5.0_f64 / 864.0_f64 * t14080 * t5861 - 7.0_f64 / 54.0_f64 * t973 * t10214 * t68521 - t973 * t977 * t68534 / 144.0_f64 + t973 * t2979 * t68539 / 216.0_f64 - t1041 * t4582 * t47775 * t70330 / 192.0_f64 + t1041 * t4582 * t14164 * t70339 / 256.0_f64 - t70597 / 1536.0_f64;
    (t70554, t70599)
}
