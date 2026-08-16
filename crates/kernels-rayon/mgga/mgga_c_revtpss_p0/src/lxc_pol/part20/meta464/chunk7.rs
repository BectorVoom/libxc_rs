//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1771/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1771(t10061: f64, t10069: f64, t2782: f64, t4086: f64, t46407: f64, t543: f64, t4003: f64, t46565: f64, t5744: f64, t10073: f64, t10111: f64, t1428: f64, t588: f64) -> (f64, f64, f64, f64, f64) {
    let t47403 = t10069 * t10061;
    let t47407 = t2782 * t4086 * t46407 * t543;
    let t47411 = t2782 * t5744 * t46565 * t4003;
    let t47413 = t10073 * t10061;
    let t47417 = 0.15709759505761725819e-2_f64 * t10111 * t1428 * t588;
    (t47403, t47407, t47411, t47413, t47417)
}
