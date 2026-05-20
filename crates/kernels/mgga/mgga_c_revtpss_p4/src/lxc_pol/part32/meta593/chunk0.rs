//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1925/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1925<F: Float>(t99033: F, t99041: F, t99066: F, t99069: F, t99073: F, t99077: F, t99085: F, t99099: F, t99102: F, t136: F, t2457: F, t8015: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t103296 = F::cast_from(0.80031500487063509014e-2_f64) * t99033;
    let t103301 = F::cast_from(0.22866142996303859718e-3_f64) * t99041;
    let t103315 = F::cast_from(0.16006300097412701803e0_f64) * t99066;
    let t103316 = F::cast_from(0.11433071498151929859e-3_f64) * t99069;
    let t103318 = F::cast_from(0.2032800112371413129e-2_f64) * t99073;
    let t103320 = F::cast_from(0.10164000561857065645e-3_f64) * t99077;
    let t103324 = F::cast_from(0.2032800112371413129e-3_f64) * t99085;
    let t103336 = F::new(7.0) / F::new(36.0) * t99099;
    let t103337 = F::new(7.0) / F::new(12.0) * t99102;
    let t103363 = t8015 * t136 * t2457;
    (t103296, t103301, t103315, t103316, t103318, t103320, t103324, t103336, t103337, t103363)
}
