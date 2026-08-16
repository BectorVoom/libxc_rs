//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1168/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1168(t23164: f64, t23204: f64, t23222: f64, t23168: f64, t23238: f64, t22986: f64, t23270: f64, t2553: f64, t857: f64, t865: f64, t23196: f64, t6562: f64) -> (f64, f64, f64, f64) {
    let t82172 = t23164 * t23204 * t23222;
    let t82174 = t23168 * t23238;
    let t82179 = t22986 * t23270 * t857 * t2553 * t865;
    let t82182 = t6562 * t23204 * t23196;
    (t82172, t82174, t82179, t82182)
}
