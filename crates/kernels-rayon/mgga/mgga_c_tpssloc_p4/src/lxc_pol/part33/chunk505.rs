//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 505/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk505(t1878: f64, t268: f64, t405: f64, t1229: f64, t154: f64, t636: f64) -> (f64, f64, f64, f64, f64) {
    let t3236 = t268 * t1878 * t405;
    let t3237 = 0.23744444444444444444e-1_f64 * t3236;
    let t3240 = t154 * t1229;
    let t3241 = t636 * t636;
    let t3242 = 1.0_f64 / t3241;
    (t3236, t3237, t3240, t3241, t3242)
}
