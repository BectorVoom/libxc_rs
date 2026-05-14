//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1181/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1181<F: Float>(t11431: F, t51306: F, t11854: F, t14031: F, t11860: F, t4028: F, t11919: F, t4049: F, t51459: F, t54398: F, t54402: F, t57213: F, t57216: F, t57219: F, t57223: F, t57225: F, t57227: F) -> (F,) {
    let t57229 = t51306 * t11431;
    let t57231 = t14031 * t11854;
    let t57233 = t4028 * t11860;
    let t57235 = t4049 * t11919;
    let t57237 = -t51459 + 7.0 / 576.0 * t57213 + t54398 - t54402 + t57216 / 96.0 - t57219 / 48.0 - t57223 / 96.0 + t57225 / 64.0 + t57227 / 384.0 + t57229 / 48.0 - t57231 / 384.0 + t57233 / 48.0 + 5.0 / 192.0 * t57235;
    (t57237,)
}
