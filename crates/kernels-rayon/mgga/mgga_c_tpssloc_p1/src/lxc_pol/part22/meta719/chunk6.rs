//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2333/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333(t20974: f64, t9638: f64, t20891: f64, t120: f64, t20800: f64, t20904: f64, t41414: f64, t13177: f64, t13251: f64, t16673: f64, t16898: f64, t20756: f64, t2643: f64, t2645: f64, t40966: f64, t40971: f64, t4177: f64, t4184: f64, t4250: f64, t46546: f64, t5619: f64, t58421: f64, t58425: f64, t58427: f64, t58642: f64, t776: f64, t820: f64, t829: f64, t843: f64) -> f64 {
    let t67637 = t9638 * t20974;
    let t67639 = t9638 * t20891;
    let t67644 = t120 * t20800;
    let t67660 = t41414 * t20904;
    let t67667 = 35.0_f64 / 384.0_f64 * t67637 + 7.0_f64 / 1536.0_f64 * t67639 + t16673 * t4177 * t4184 / 512.0_f64 + t2643 * t2645 * t67644 * t829 / 768.0_f64 - 5.0_f64 / 256.0_f64 * t13251 * t16898 + t58642 * t4250 / 256.0_f64 + 595.0_f64 / 2592.0_f64 * t40966 + 455.0_f64 / 216.0_f64 * t46546 + 119.0_f64 / 576.0_f64 * t58421 + 7.0_f64 / 192.0_f64 * t58425 - 7.0_f64 / 384.0_f64 * t58427 - t13177 * t5619 / 1024.0_f64 + 7.0_f64 / 768.0_f64 * t67660 + 35.0_f64 / 128.0_f64 * t843 * t40971 * t820 * t20756 * t776;
    t67667
}
