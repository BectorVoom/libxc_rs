//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2617/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2617(t22243: f64, t486: f64, t1222: f64, t22116: f64, t1216: f64, t1227: f64, t15615: f64, t1743: f64, t18573: f64, t22197: f64, t3490: f64, t3506: f64, t3515: f64, t4582: f64, t488: f64, t4978: f64, t51002: f64, t53271: f64, t53273: f64, t53274: f64, t66449: f64, t66452: f64, t66458: f64, t70330: f64, t70339: f64) -> f64 {
    let t73028 = t486 * t22243;
    let t73043 = t22116 * t1222;
    let t73048 = -t1227 * t4582 * t15615 * t70339 / 256.0_f64 + t53271 - t53273 - t53274 / 648.0_f64 - t66449 / 72.0_f64 - t66452 / 48.0_f64 - 2.0_f64 / 81.0_f64 * t66458 + t3506 * t4582 * t73028 * t4978 / 1536.0_f64 - t3515 * t4582 * t73028 * t1216 / 3072.0_f64 + 5.0_f64 / 384.0_f64 * t1227 * t4582 * t51002 * t70330 + 5.0_f64 / 4608.0_f64 * t3490 * t22197 + t73043 / 4608.0_f64 - t18573 * t1743 * t488 / 192.0_f64;
    t73048
}
