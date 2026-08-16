//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1463/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1463(t4147: f64, t6922: f64, t566: f64, t6816: f64, t1448: f64, t1868: f64, t1353: f64, t13664: f64, t13682: f64, t13683: f64, t198: f64, t22214: f64, t22215: f64, t22216: f64, t22217: f64, t22218: f64, t22219: f64, t4139: f64, t4140: f64, t5536: f64, t5541: f64, t5542: f64, t5778: f64, t6836: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64) -> f64 {
    let t22483 = t6922 * t4147;
    let t22486 = t566 * t6816;
    let t22496 = t1868 * t1448;
    let t22504 = 6.0_f64 * t1353 * t198 * t566 * t6836 + 6.0_f64 * t1353 * t22486 * t5536 - t1448 * t22483 * t5541 - 6.0_f64 * t22496 * t4139 * t5542 + 3.0_f64 * t4139 * t4140 * t6816 - 2.0_f64 * t5541 * t5542 * t5778 - t13664 + t13682 + t13683 - t22214 + t22215 - t22216 - t22217 + t22218 + t22219 - t9524 + t9542 + t9854 + t9865 + t9868;
    t22504
}
