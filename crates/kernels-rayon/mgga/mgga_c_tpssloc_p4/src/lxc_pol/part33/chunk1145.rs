//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1145/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1145(t26251: f64, t26255: f64, t26266: f64, t26361: f64, t26393: f64, t26406: f64, t26429: f64, t26127: f64, t19299: f64, t33: f64, t22505: f64, t22510: f64, t5392: f64, t5398: f64, t6500: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27019 = 7.0_f64 / 1152.0_f64 * t26251;
    let t27022 = 7.0_f64 / 288.0_f64 * t26255;
    let t27027 = 7.0_f64 / 72.0_f64 * t26266;
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27082 = 0.16449340668482264365e-1_f64 * t26393;
    let t27088 = 0.38381794893125283518e-1_f64 * t26406;
    let t27096 = 0.38381794893125283518e-1_f64 * t26429;
    let t27166 = 2.0_f64 / 3.0_f64 * t26127;
    let t27937 = t19299 * t33;
    let t27948 = 5.0_f64 / 18.0_f64 * t22505 * t5392 + 5.0_f64 / 6.0_f64 * t6500 * t5398 - t22510;
    (t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27937, t27948)
}
