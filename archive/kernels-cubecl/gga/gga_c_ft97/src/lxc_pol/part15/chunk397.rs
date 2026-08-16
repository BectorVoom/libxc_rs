//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 397/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk397<F: Float>(t2: F, t942: F, t2981: F, t3006: F, t376: F, t89: F, t973: F, t103: F, t1570: F, t100: F, t1780: F, t1557: F) -> (F, F, F, F, F, F, F) {
    let t3149 = t2 * t942;
    let t3161 = t2981 / F::cast_from(27.0_f64);
    let t3166 = t3006 / F::cast_from(9.0_f64);
    let t3177 = t89 * t376 * t973;
    let t3187 = t103 * t1570;
    let t3193 = t1780 * t100;
    let t3194 = t103 * t1557;
    (t3149, t3161, t3166, t3177, t3187, t3193, t3194)
}
