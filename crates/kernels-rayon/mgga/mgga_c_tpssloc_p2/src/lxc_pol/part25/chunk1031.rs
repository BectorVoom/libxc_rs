//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1031/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1031(t23110: f64, t6648: f64, t23185: f64, t225: f64, t2717: f64, t2719: f64, t6553: f64, t1880: f64, t252: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23186 = t23110 * t6648;
    let t23187 = t23185 * t23186;
    let t23195 = t225 * t2717;
    let t23196 = t23195 * t2719;
    let t23197 = t6553 * t23196;
    let t23198 = t1880 * t23197;
    let t23204 = t794 * t252;
    (t23186, t23187, t23196, t23197, t23198, t23204)
}
