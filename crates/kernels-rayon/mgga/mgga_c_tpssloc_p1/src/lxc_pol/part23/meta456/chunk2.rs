//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1321/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321(t232: f64, t76085: f64, t119: f64, t1484: f64, t16872: f64, t20800: f64, t20904: f64, t20949: f64, t210: f64, t2630: f64, t2701: f64, t41139: f64, t41349: f64, t4172: f64, t46957: f64, t47047: f64, t5614: f64, t5619: f64, t68021: f64, t75978: f64, t76002: f64, t76074: f64, t76086: f64, t787: f64, t817: f64, t819: f64, t820: f64, t843: f64) -> (f64, f64) {
    let t76327 = t76085 * t232;
    let t76333 = -t46957 * t20904 / 128.0_f64 + 5.0_f64 / 64.0_f64 * t4172 * t20949 - t16872 * t5614 / 512.0_f64 - t787 * t210 * t119 * t75978 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t68021 + t2630 * t819 * t820 * t76002 / 512.0_f64 + t41349 * t819 * t820 * t76086 / 128.0_f64 - t16872 * t5619 / 512.0_f64 + 5.0_f64 / 192.0_f64 * t843 * t2701 * t820 * t20800 * t1484 - t817 * t819 * t820 * t76074 / 3072.0_f64 - t817 * t819 * t820 * t76327 / 3072.0_f64 - 595.0_f64 / 2592.0_f64 * t47047 + t41139;
    (t76327, t76333)
}
