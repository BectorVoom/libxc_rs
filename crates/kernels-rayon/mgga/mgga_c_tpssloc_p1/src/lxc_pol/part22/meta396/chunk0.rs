//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1688/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1688(t11721: f64, t1215: f64, t18300: f64, t4582: f64, t4978: f64, t1222: f64, t6170: f64, t6158: f64, t6165: f64, t11644: f64, t11649: f64, t11719: f64, t11728: f64, t15446: f64, t15448: f64, t15450: f64, t15452: f64, t15503: f64, t15507: f64, t18297: f64, t488: f64, t4974: f64, t4980: f64, t4984: f64, t5005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18301 = t11721 * t1215;
    let t18302 = t18300 * t18301;
    let t18303 = t4582 * t18302;
    let t18306 = t18300 * t4978;
    let t18307 = t4582 * t18306;
    let t18310 = t6170 * t1222;
    let t18312 = t6158 * t1222;
    let t18314 = t6165 * t1222;
    let t18316 = -t11644 / 13824.0_f64 + t11649 - t15503 * t4980 / 144.0_f64 + t15507 * t4984 / 288.0_f64 - t5005 * t4974 / 1152.0_f64 - t18297 * t488 / 288.0_f64 + t11719 * t18303 / 512.0_f64 - t11728 * t18307 / 512.0_f64 + t15446 - t15448 - t15450 + t15452 + t18310 / 4608.0_f64 - t18312 / 432.0_f64 + 19.0_f64 / 2592.0_f64 * t18314;
    (t18301, t18302, t18303, t18306, t18307, t18310, t18312, t18314, t18316)
}
