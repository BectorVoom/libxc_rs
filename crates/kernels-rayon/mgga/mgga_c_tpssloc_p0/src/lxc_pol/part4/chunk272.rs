//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 272/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk272(t829: f64, t860: f64, t235: f64, t852: f64, t226: f64, t255: f64, t808: f64, t812: f64, t858: f64, t259: f64, t799: f64, t853: f64, t855: f64) -> (f64, f64, f64, f64, f64) {
    let t861 = t860 * t829;
    let t863 = t235 * t852;
    let t865 = t226 * t863 + t255 * t808 - t812 * t861;
    let t866 = t858 * t865;
    let t868 = t259 * t799 + t259 * t853 - t855 * t866;
    (t861, t863, t865, t866, t868)
}
