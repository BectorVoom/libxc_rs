//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1376/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1376<F: Float>(t127680: F, t31537: F, t123503: F, t31462: F, t112358: F, t112365: F, t127306: F, t127310: F, t127582: F, t127650: F, t127651: F, t127655: F, t127659: F, t127663: F, t127666: F, t19039: F, t31411: F, t31515: F, t5265: F, t54924: F, t54928: F, t7003: F, t812: F, t821: F, t98612: F) -> (F,) {
    let t127681 = t31537 * t127680;
    let t127684 = t31462 * t123503;
    let t127688 = -0.28251445438725559514e-1 * t127650 * t127651 + 0.28251445438725559514e-1 * t31537 * t127655 - t112358 + 0.4833552354146973393e0 * t127659 * t31411 - 0.12220869211492952596e0 * t127663 * t812 + 0.61104346057464762978e-1 * t127666 * t821 + 0.90613700826057446696e0 * t54924 * t127306 - 0.13592055123908617004e1 * t54928 * t127310 + 0.93056218143801431978e1 * t19039 * t31515 * t812 - 0.46528109071900715989e1 * t5265 * t31515 * t821 - 0.47085742397875932523e-2 * t127681 + 0.3704333394032921811e-2 * t98612 + t112365 + 0.18834296959150373009e-1 * t127684 - 0.12220869211492952596e0 * t7003 * t127582;
    (t127688,)
}
