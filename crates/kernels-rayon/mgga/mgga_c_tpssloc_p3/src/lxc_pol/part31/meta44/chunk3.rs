//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 305/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk305(t829: f64, t860: f64, t235: f64, t852: f64, t226: f64, t255: f64, t808: f64, t812: f64) -> (f64, f64, f64) {
    let t861 = t860 * t829;
    let t863 = t235 * t852;
    let t865 = t226 * t863 + t255 * t808 - t812 * t861;
    (t861, t863, t865)
}
