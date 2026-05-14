//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 393/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk393<F: Float>(t103: F, t1570: F, t100: F, t1780: F, t1557: F, t1882: F, t981: F, t487: F, t971: F) -> (F, F, F, F, F) {
    let t3187 = t103 * t1570;
    let t3193 = t1780 * t100;
    let t3194 = t103 * t1557;
    let t3224 = t1882 * t981;
    let t3238 = t971 * t487;
    (t3187, t3193, t3194, t3224, t3238)
}
