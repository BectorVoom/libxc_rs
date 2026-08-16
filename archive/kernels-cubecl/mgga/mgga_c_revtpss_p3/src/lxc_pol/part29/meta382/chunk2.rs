//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1376/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1376<F: Float>(t14334: F, t2496: F, t4398: F, t10443: F, t10552: F, t10554: F, t14312: F, t14313: F, t14315: F, t14317: F, t14318: F, t14324: F, t14327: F, t14329: F, t14333: F, t4541: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F) {
    let t14335 = F::cast_from(0.5848223622634646207e0_f64) * t14334;
    let t14336 = t4398 * t2496;
    let t14337 = F::cast_from(0.17315859105681463759e2_f64) * t14336;
    let t14338 = F::cast_from(12.0_f64) * t14318 * t4541 * t775 + t10443 - t10552 + t10554 + t14312 + t14313 + t14315 + t14317 - t14324 + t14327 + t14329 + t14333 - t14335 - t14337 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t14335, t14337, t14338)
}
