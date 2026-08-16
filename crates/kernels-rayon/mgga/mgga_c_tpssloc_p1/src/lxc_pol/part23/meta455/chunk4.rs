//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1317/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317(t1516: f64, t16976: f64, t20896: f64, t20908: f64, t40971: f64, t4172: f64, t46577: f64, t5624: f64, t5628: f64, t58550: f64, t67690: f64, t67692: f64, t67729: f64, t67735: f64, t68203: f64, t75978: f64, t76056: f64, t820: f64, t843: f64, t847: f64) -> f64 {
    let t76193 = 35.0_f64 / 128.0_f64 * t843 * t40971 * t820 * t76056 - 5.0_f64 / 32.0_f64 * t4172 * t20896 - t4172 * t20908 / 192.0_f64 - t843 * t847 * t820 * t75978 / 768.0_f64 + 5.0_f64 / 128.0_f64 * t16976 * t5624 - t16976 * t5628 / 128.0_f64 - t68203 * t1516 / 192.0_f64 - 7.0_f64 / 96.0_f64 * t67690 - 7.0_f64 / 192.0_f64 * t67692 - 7.0_f64 / 96.0_f64 * t67729 + 7.0_f64 / 1152.0_f64 * t67735 + 595.0_f64 / 648.0_f64 * t46577 - 35.0_f64 / 36.0_f64 * t58550;
    t76193
}
