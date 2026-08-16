//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1033/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1033<F: Float>(t27: F, t370: F, t85682: F, t89: F, t446: F, t7793: F, t86104: F, t38268: F, t86098: F, t1564: F, t86108: F, t86054: F) -> (F, F, F, F, F) {
    let t86289 = t89 * t27 * t370 * t85682;
    let t86297 = t446 * t7793 * t86104;
    let t86300 = t446 * t38268 * t86098;
    let t86303 = t446 * t1564 * t86108;
    let t86306 = t446 * t1564 * t86054;
    (t86289, t86297, t86300, t86303, t86306)
}
