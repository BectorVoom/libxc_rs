//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1481/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1481(t14334: f64, t2496: f64, t4398: f64, t10443: f64, t10552: f64, t10554: f64, t14312: f64, t14313: f64, t14315: f64, t14317: f64, t14318: f64, t14324: f64, t14327: f64, t14329: f64, t14333: f64, t4541: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64) {
    let t14335 = 0.5848223622634646207e0_f64 * t14334;
    let t14336 = t4398 * t2496;
    let t14337 = 0.17315859105681463759e2_f64 * t14336;
    let t14338 = 12.0_f64 * t14318 * t4541 * t775 + t10443 - t10552 + t10554 + t14312 + t14313 + t14315 + t14317 - t14324 + t14327 + t14329 + t14333 - t14335 - t14337 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t14335, t14337, t14338)
}
