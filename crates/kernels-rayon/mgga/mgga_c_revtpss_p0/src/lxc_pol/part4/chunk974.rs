//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 974/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk974(t3889: f64, t543: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64, t124: f64, t1398: f64, t3938: f64, t4003: f64, t4056: f64) -> (f64, f64, f64, f64, f64) {
    let t9810 = t543 * t3889;
    let t9816 = t2482 * t1386 * t814;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9840 = t4003 * t4056;
    (t9810, t9816, t9818, t9822, t9840)
}
