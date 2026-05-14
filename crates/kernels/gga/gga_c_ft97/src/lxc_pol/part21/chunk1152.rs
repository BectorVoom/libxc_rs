//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1152/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1152<F: Float>(t102103: F, t116340: F, t116344: F, t116347: F, t116350: F, t116354: F, t116358: F, t116363: F, t116368: F, t116373: F, t116377: F, t93728: F, t22952: F, t4436: F, t473: F, t5675: F, t8411: F) -> (F, F) {
    let t116379 = -t116340 / 3.0 + 2.0 / 9.0 * t116344 - t116347 - t116350 / 3.0 - t116354 + t116358 / 2.0 + t102103 + t116363 / 4.0 + 2.0 * t116368 + t116373 / 4.0 + t116377 / 3.0 + t93728;
    let t116383 = t22952 * t8411 * t5675 * t4436 * t473;
    (t116379, t116383)
}
