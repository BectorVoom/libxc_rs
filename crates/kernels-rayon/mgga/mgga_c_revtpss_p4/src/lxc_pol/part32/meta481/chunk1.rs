//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1723/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1723(t27384: f64, t27799: f64, t1113: f64, t1583: f64, t33: f64, t4537: f64, t1711: f64, t775: f64, t890: f64, t196: f64, t197: f64, t5528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27800 = t27799 * t27384;
    let t27802 = t1113 * t1583;
    let t27806 = t33 * t4537;
    let t27810 = t1711 * t775;
    let t27817 = t1711 * t890;
    let t27833 = t5528 * t196 * t197;
    (t27800, t27802, t27806, t27810, t27817, t27833)
}
