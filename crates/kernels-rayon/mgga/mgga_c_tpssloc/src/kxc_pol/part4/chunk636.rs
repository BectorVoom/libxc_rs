//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 636/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk636(t119: f64, t4119: f64, t210: f64, t225: f64, t4142: f64, t237: f64, t1499: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t4158 = t119 * t4119;
    let t4159 = t210 * t4158;
    let t4162 = t4142 * t225;
    let t4163 = t4162 * t237;
    let t4166 = t1499 * t68;
    (t4159, t4162, t4163, t4166)
}
