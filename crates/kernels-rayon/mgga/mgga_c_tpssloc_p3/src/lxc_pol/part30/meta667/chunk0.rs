//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2093/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2093(t5259: f64, t80820: f64, t22779: f64, t26292: f64, t16060: f64, t6944: f64, t1827: f64, t80991: f64, t22765: f64, t5289: f64, t22764: f64, t5234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91214 = t80820 * t5259;
    let t91215 = 7.0_f64 / 288.0_f64 * t91214;
    let t91225 = t22779 * t26292;
    let t91226 = 0.28260929265898273598e-2_f64 * t91225;
    let t91278 = t16060 * t6944;
    let t91281 = t80991 * t1827;
    let t91282 = 7.0_f64 / 1152.0_f64 * t91281;
    let t91283 = t22765 * t5289;
    let t91284 = 7.0_f64 / 1152.0_f64 * t91283;
    let t91285 = t5234 * t22764;
    (t91215, t91226, t91278, t91282, t91284, t91285)
}
