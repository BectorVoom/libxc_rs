//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1468/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1468(t11719: f64, t11728: f64, t11738: f64, t1227: f64, t15438: f64, t15659: f64, t15737: f64, t1735: f64, t1743: f64, t19056: f64, t22115: f64, t22271: f64, t22275: f64, t22314: f64, t248: f64, t3506: f64, t3515: f64, t3585: f64, t4582: f64, t488: f64, t53472: f64, t6225: f64, t6230: f64, t65474: f64, t66015: f64, t72669: f64, t72673: f64, t73028: f64, t77965: f64) -> f64 {
    let t79160 = -7.0_f64 / 486.0_f64 * t72669 - t22115 * t1743 * t488 / 144.0_f64 - t72673 / 72.0_f64 + t15737 * t22271 / 128.0_f64 + t3506 * t4582 * t73028 * t15659 / 384.0_f64 + 3.0_f64 / 256.0_f64 * t11719 * t4582 * t19056 * t65474 - 3.0_f64 / 256.0_f64 * t11728 * t4582 * t19056 * t6225 - t15438 * t22275 / 256.0_f64 - t3515 * t4582 * t73028 * t1735 / 768.0_f64 + t11738 * t4582 * t19056 * t6230 / 512.0_f64 + t66015 / 108.0_f64 + 5.0_f64 / 4608.0_f64 * t1227 * t248 * t3585 * t77965 - t53472 * t22314 / 128.0_f64;
    t79160
}
