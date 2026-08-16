//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2218/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218(t14507: f64, t23540: f64, t23433: f64, t4630: f64, t10189: f64, t1920: f64, t4343: f64, t13783: f64, t4338: f64, t13546: f64, t13555: f64, t13559: f64, t14099: f64, t14103: f64, t14167: f64, t1618: f64, t23541: f64, t25571: f64, t25574: f64, t2987: f64, t3043: f64, t4509: f64, t6680: f64, t6765: f64, t82964: f64) -> f64 {
    let t88600 = t14507 * t23540;
    let t88604 = t23433 * t4630 / 1152.0_f64;
    let t88622 = t1920 * t10189 * t4343 / 216.0_f64;
    let t88625 = t1920 * t13783 * t4338 / 324.0_f64;
    let t88632 = -t88600 * t3043 / 1536.0_f64 + t88604 + 19.0_f64 / 864.0_f64 * t82964 * t1618 + t1920 * t2987 * t13559 / 48.0_f64 - t23541 * t14099 / 768.0_f64 - t23541 * t14103 / 1536.0_f64 + t6765 * t14167 / 384.0_f64 + t6680 * t25571 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t6680 * t25574 - t88622 + t88625 - t1920 * t2987 * t13546 / 144.0_f64 - t1920 * t4509 * t13555 / 36.0_f64;
    t88632
}
