//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1362/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1362<F: Float>(t24378: F, t25070: F, t31450: F, t4917: F, t811: F, t820: F, t1091: F, t111910: F, t112015: F, t112016: F, t112018: F, t112021: F, t112033: F, t112055: F, t112268: F, t1208: F, t122827: F, t123709: F, t127147: F, t231: F, t2441: F, t25049: F, t25077: F, t28552: F, t4088: F, t6035: F, t6045: F) -> (F, F, F) {
    let t127218 = t25070 * t24378 * t31450;
    let t127234 = t4917 * t811;
    let t127239 = t4917 * t820;
    let t127244 = 0.40006800655555555556e0 * t25049 * t6045 * t231 * t4088 * t1208 - 0.22226000364197530865e-1 * t127218 + 0.9667104708293946786e0 * t112268 * t127147 - 0.51860667516460905352e-1 * t28552 * t123709 - t112015 + 0.7408666788065843622e-2 * t112016 - 0.1611184118048991131e0 * t112018 + 0.1611184118048991131e0 * t112021 - 0.1611184118048991131e0 * t112033 - 0.13335600218518518519e0 * t28552 * t122827 + 0.66678001092592592595e-1 * t25077 * t6035 * t111910 * t1091 - 0.4445200072839506173e-1 * t25070 * t6035 * t2441 * t127234 + 0.4445200072839506173e-1 * t25077 * t6035 * t2441 * t127239 + t112055;
    (t127234, t127239, t127244)
}
