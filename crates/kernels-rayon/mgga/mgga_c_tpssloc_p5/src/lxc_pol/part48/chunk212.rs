//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 212/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk212(t252: f64, t814: f64, t829: f64, t235: f64, t852: f64, t226: f64, t255: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t860 = t814 * t252;
    let t861 = t860 * t829;
    let t863 = t235 * t852;
    let t865 = t226 * t863 + t255 * t808 - t812 * t861;
    (t860, t861, t863, t865)
}
