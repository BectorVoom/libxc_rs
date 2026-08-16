//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1290/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1290(t18464: f64, t4484: f64, t1646: f64, t60749: f64, t60750: f64, t19506: f64, t5570: f64, t18495: f64, t6259: f64, t20509: f64, t2436: f64, t6353: f64, t8096: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65643 = t18464 * t4484;
    let t65647 = t60749 * t1646;
    let t65650 = 119.0_f64 / 864.0_f64 * t60750;
    let t65667 = t19506 * t5570;
    let t65871 = t6259 * t18495;
    let t66281 = t20509 * t2436;
    let t66299 = t6353 * t8096;
    (t65643, t65647, t65650, t65667, t65871, t66281, t66299)
}
