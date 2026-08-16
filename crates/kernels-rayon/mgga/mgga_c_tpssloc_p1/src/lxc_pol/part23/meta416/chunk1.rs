//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1235/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1235(t20994: f64, t2563: f64, t20944: f64, t41011: f64, t13278: f64, t5614: f64, t20963: f64, t9667: f64, t46881: f64, t5587: f64, t20908: f64, t2697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67920 = t2563 * t20994;
    let t67937 = t41011 * t20944;
    let t67976 = t13278 * t5614;
    let t67978 = t9667 * t20963;
    let t67980 = t46881 * t5587;
    let t68021 = t2697 * t20908;
    (t67920, t67937, t67976, t67978, t67980, t68021)
}
