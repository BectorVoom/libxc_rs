//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2252/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252(t13176: f64, t2638: f64, t831: f64, t13251: f64, t13350: f64, t2643: f64, t2645: f64, t2647: f64, t41048: f64, t41050: f64, t41053: f64, t41055: f64, t41063: f64, t4191: f64, t4248: f64, t4257: f64, t46644: f64, t46650: f64, t46658: f64, t46661: f64, t46663: f64, t9623: f64, t9661: f64, t9990: f64) -> f64 {
    let t46667 = t13176 * t2638;
    let t46668 = t46667 * t831;
    let t46670 = t2643 * t2645 * t4248 * t9661 / 768.0_f64 - 7.0_f64 / 384.0_f64 * t41048 - 7.0_f64 / 384.0_f64 * t41050 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t46644 * t2647 + t46650 + t41063 * t4191 / 256.0_f64 - 119.0_f64 / 576.0_f64 * t41053 + 7.0_f64 / 384.0_f64 * t41055 - t13251 * t9623 / 1024.0_f64 - 7.0_f64 / 192.0_f64 * t46658 - 7.0_f64 / 384.0_f64 * t46661 - 35.0_f64 / 192.0_f64 * t46663 + 5.0_f64 / 256.0_f64 * t9990 * t4257 + 7.0_f64 / 768.0_f64 * t46668;
    t46670
}
