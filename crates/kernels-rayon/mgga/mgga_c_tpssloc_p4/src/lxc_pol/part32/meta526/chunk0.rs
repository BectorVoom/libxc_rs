//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1860/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1860(t26231: f64, t26251: f64, t26255: f64, t26266: f64, t26361: f64, t26393: f64, t26406: f64, t26429: f64, t26127: f64, t2165: f64, t4072: f64, t671: f64, t8103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27012 = 7.0_f64 / 1152.0_f64 * t26231;
    let t27019 = 7.0_f64 / 1152.0_f64 * t26251;
    let t27022 = 7.0_f64 / 288.0_f64 * t26255;
    let t27027 = 7.0_f64 / 72.0_f64 * t26266;
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27082 = 0.16449340668482264365e-1_f64 * t26393;
    let t27088 = 0.38381794893125283518e-1_f64 * t26406;
    let t27096 = 0.38381794893125283518e-1_f64 * t26429;
    let t27166 = 2.0_f64 / 3.0_f64 * t26127;
    let t27290 = t2165 * t4072;
    let t27293 = t8103 * t671;
    (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27290, t27293)
}
