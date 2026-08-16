//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 898/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk898(t1799: f64, t212: f64, t12199: f64, t5202: f64, t12225: f64, t2586: f64, t2371: f64, t5154: f64, t12365: f64, t1827: f64, t3789: f64, t5234: f64) -> (f64, f64, f64, f64, f64) {
    let t16095 = t212 * t1799;
    let t16108 = t12199 * t5202;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    let t16164 = t5154 * t2371;
    let t16211 = t12365 * t1827;
    let t16285 = t5234 * t3789;
    (t16108, t16119, t16164, t16211, t16285)
}
