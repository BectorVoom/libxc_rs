//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1132/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1132(t23155: f64, t23168: f64, t6552: f64, t6637: f64, t6638: f64, t9516: f64, t22893: f64, t23158: f64, t23164: f64, t22715: f64, t6551: f64, t6640: f64) -> (f64, f64, f64, f64, f64) {
    let t81623 = t23168 * t23155;
    let t81627 = t6552 * t6637 * t6638 * t9516;
    let t81630 = t23164 * t22893 * t23158;
    let t81632 = t22715 * t6551;
    let t81633 = t81632 * t6640;
    (t81623, t81627, t81630, t81632, t81633)
}
