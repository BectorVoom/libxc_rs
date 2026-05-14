//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 984/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk984<F: Float>(t30472: F, t574: F, t605: F, t1359: F, t4724: F, t2179: F, t167: F, t30232: F, t9432: F, t144: F, t30304: F, t1053: F, t6615: F, t4805: F, t143: F, t160: F, t30280: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30474 = t574 * t605 * t30472;
    let t30477 = t1359 * t4724;
    let t30479 = t574 * t2179 * t30477;
    let t30483 = t9432 * t167 * t30232;
    let t30486 = t144 * t30304;
    let t30489 = t6615 * t1053;
    let t30491 = t574 * t605 * t30489;
    let t30494 = t1359 * t4805;
    let t30496 = t574 * t605 * t30494;
    let t30500 = t143 * t30280 * t160;
    (t30474, t30477, t30479, t30483, t30486, t30489, t30491, t30494, t30496, t30500)
}
