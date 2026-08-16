//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1320/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320(t232: f64, t76001: f64, t2632: f64, t76085: f64, t13283: f64, t1510: f64, t20963: f64, t20969: f64, t20981: f64, t2630: f64, t2643: f64, t2645: f64, t41096: f64, t4167: f64, t4178: f64, t5527: f64, t5544: f64, t58809: f64, t67607: f64, t67644: f64, t67976: f64, t67978: f64, t67980: f64, t76090: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9974: f64) -> (f64, f64, f64) {
    let t76274 = t76001 * t232;
    let t76290 = t76085 * t2632;
    let t76295 = -t4178 * t2645 * t67607 * t20981 / 32.0_f64 + t2643 * t2645 * t67644 * t1510 / 192.0_f64 + 7.0_f64 / 384.0_f64 * t67976 - 7.0_f64 / 192.0_f64 * t67978 - 7.0_f64 / 192.0_f64 * t67980 + t41096 + 119.0_f64 / 1152.0_f64 * t58809 - t4167 * t20969 / 768.0_f64 - t817 * t819 * t820 * t76274 / 1024.0_f64 + t13283 * t20963 / 128.0_f64 - 15.0_f64 / 64.0_f64 * t843 * t9607 * t820 * t5527 * t5544 - 3.0_f64 / 256.0_f64 * t9974 * t819 * t820 * t76090 + 7.0_f64 / 1536.0_f64 * t2630 * t819 * t820 * t76290;
    (t76274, t76290, t76295)
}
