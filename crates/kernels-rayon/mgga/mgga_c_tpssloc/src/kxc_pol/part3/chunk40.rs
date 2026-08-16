//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 40/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk40(t101: f64, t103: f64, t100: f64, t92: f64, t96: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t104 = t103 * t101;
    let t106 = t100 * t104 + t92 * t96;
    let t107 = 1.0_f64 / t106;
    let t109 = t64 * t107 / 8.0_f64;
    let t110 = 1.0_f64 < t109;
    let t111 = piecewise3(t110, 1.0_f64, t109);
    (t104, t106, t107, t111, t109)
}
