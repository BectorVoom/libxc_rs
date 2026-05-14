//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1077/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1077<F: Float>(t1882: F, t26259: F, t26159: F, t8392: F, t6549: F, t8232: F, t26402: F, t26476: F, t376: F, t89: F, t26207: F, t370: F, t8418: F, t26392: F, t26280: F, t26199: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t103556 = 4.0 / 9.0 * t1882 * t26259;
    let t103571 = 2.0 / 27.0 * t8392 * t26159;
    let t103572 = t8232 * t6549;
    let t103592 = 2.0 / 9.0 * t1882 * t26402;
    let t103607 = 2.0 / 9.0 * t89 * t376 * t26476;
    let t103625 = 2.0 / 27.0 * t1882 * t26207;
    let t103626 = t370 * t8418;
    let t103632 = 2.0 / 9.0 * t1882 * t26392;
    let t103640 = 2.0 / 9.0 * t1882 * t26280;
    let t103647 = 2.0 / 27.0 * t8392 * t26199;
    (t103556, t103571, t103572, t103592, t103607, t103625, t103626, t103632, t103640, t103647)
}
