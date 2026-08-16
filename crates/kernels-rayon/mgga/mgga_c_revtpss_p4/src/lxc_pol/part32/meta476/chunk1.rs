//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1711/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1711(t4292: f64, t94: f64, t1353: f64, t1907: f64, t30: f64, t892: f64, t4433: f64, t18875: f64, t25207: f64, t1544: f64, t605: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27126 = t94 * t4292;
    let t27153 = t1907 * t1353;
    let t27159 = t892 * t30;
    let t27160 = t27159 * t4433;
    let t27166 = t25207 * t18875;
    let t27169 = t605 * t1544;
    let t27173 = t30 * t4343;
    (t27126, t27153, t27159, t27160, t27166, t27169, t27173)
}
