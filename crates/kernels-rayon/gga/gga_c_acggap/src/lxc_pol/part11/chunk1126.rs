//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1126/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1126(t2450: f64, t7646: f64, t4737: f64, t30468: f64, t4741: f64, t30216: f64, t8526: f64, t1983: f64, t30692: f64, t7586: f64, t8901: f64, t1992: f64, t7585: f64, t8906: f64) -> (f64, f64, f64, f64, f64) {
    let t35466 = t2450 * t7646;
    let t35467 = t35466 * t4737;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    let t35475 = t30692 * t7586 * t1983 * t8901;
    let t35476 = 0.7145669686344956162e-3_f64 * t35475;
    let t35479 = t7585 * t7586 * t1992 * t8906;
    (t35467, t35469, t35471, t35476, t35479)
}
