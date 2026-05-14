//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1386/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1386<F: Float>(t1359: F, t9438: F, t27001: F, t8392: F, t26520: F, t9276: F, t104381: F, t104624: F, t106807: F, t107311: F, t12338: F, t12561: F, t12590: F, t12605: F, t12754: F, t12968: F, t13065: F, t13070: F, t13140: F, t13153: F, t13204: F, t1384: F, t1391: F, t144: F, t1901: F, t23455: F, t23559: F, t27015: F, t446: F, t47659: F, t47666: F, t574: F, t605: F, t9432: F, t95837: F, t96160: F, t96167: F) -> (F, F) {
    let t107448 = t9438 * t1359;
    let t107470 = 4.0 / 3.0 * t8392 * t27001;
    let t107471 = t9276 * t26520;
    let t107475 = t446 * t574 * t605 * t1384 * t12561 / 3.0 - 8.0 / 27.0 * t96160 - 2.0 / 3.0 * t446 * t144 * t104624 - 2.0 * t446 * t9432 * t1391 * t13070 + 8.0 / 27.0 * t96167 - 2.0 / 9.0 * t1901 * t13153 * t23559 + 4.0 / 3.0 * t1901 * t12968 * t27015 * t13065 + 2.0 * t1901 * t13140 * t107448 * t12605 - 2.0 / 3.0 * t1901 * t13140 * t23455 * t12754 + 4.0 / 9.0 * t47659 * t95837 * t13204 - 8.0 / 27.0 * t47666 * t106807 * t12590 + 4.0 / 9.0 * t47659 * t107311 * t12338 - 2.0 * t446 * t144 * t104381 + t107470 + 4.0 / 3.0 * t446 * t144 * t107471;
    (t107471, t107475)
}
