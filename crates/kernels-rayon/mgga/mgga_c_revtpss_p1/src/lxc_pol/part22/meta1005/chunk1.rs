//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3437/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437(t4733: f64, t64504: f64, t981: f64, t19049: f64, t3034: f64, t19045: f64, t300: f64, t983: f64, t63940: f64, t63943: f64, t64327: f64, t64329: f64, t64488: f64, t64491: f64, t64493: f64, t64496: f64, t64498: f64, t64500: f64, t64503: f64) -> (f64, f64, f64, f64) {
    let t64507 = 0.34631718211362927518e2_f64 * t981 * t64504 * t4733;
    let t64509 = 0.17315859105681463759e2_f64 * t19049 * t3034;
    let t64510 = t300 * t19045;
    let t64512 = 0.11696447245269292414e1_f64 * t64510 * t983;
    let t64513 = -t63940 - t63943 + t64488 - t64491 + t64493 + t64327 + t64496 - t64329 - t64498 + t64500 - t64503 - t64507 - t64509 - t64512;
    (t64507, t64509, t64512, t64513)
}
