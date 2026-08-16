//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2329/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2329(t16215: f64, t221: f64, t91194: f64, t6604: f64, t80893: f64, t1361: f64, t6925: f64, t6976: f64, t22828: f64, t26243: f64, t26271: f64, t80779: f64) -> (f64, f64, f64, f64) {
    let t91196 = t91194 * t221 * t16215;
    let t91198 = t80893 * t6604;
    let t91200 = t91198 * t1361 * t16215;
    let t91202 = t6925 * t6976;
    let t91204 = t91202 * t26243 * t22828;
    let t91206 = t80779 * t26271;
    (t91196, t91200, t91204, t91206)
}
