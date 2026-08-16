//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1237/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237(t41414: f64, t9978: f64, t9667: f64, t9983: f64, t2617: f64, t9666: f64, t2635: f64, t2639: f64, t9663: f64, t232: f64, t41367: f64, t2630: f64, t2681: f64, t2701: f64, t40926: f64, t41395: f64, t41397: f64, t41399: f64, t41404: f64, t41410: f64, t776: f64, t817: f64, t819: f64, t820: f64, t831: f64, t843: f64, t9516: f64, t9613: f64) -> (f64, f64) {
    let t41415 = t41414 * t9978;
    let t41417 = t9667 * t9983;
    let t41424 = t2617 * t9666;
    let t41425 = t41424 * t2635;
    let t41427 = t2639 * t9663;
    let t41429 = t41367 * t232;
    let t41434 = 7.0_f64 / 384.0_f64 * t41395 - 35.0_f64 / 96.0_f64 * t41397 - t41399 * t831 / 768.0_f64 - t9613 * t2681 / 512.0_f64 + 7.0_f64 / 384.0_f64 * t41404 + 7.0_f64 / 1536.0_f64 * t2630 * t819 * t820 * t40926 + t41410 * t2635 / 256.0_f64 + 7.0_f64 / 192.0_f64 * t41415 - 7.0_f64 / 192.0_f64 * t41417 + 5.0_f64 / 192.0_f64 * t843 * t2701 * t820 * t9516 * t776 - 7.0_f64 / 192.0_f64 * t41425 + 7.0_f64 / 1152.0_f64 * t41427 - t817 * t819 * t820 * t41429 / 1024.0_f64;
    (t41429, t41434)
}
