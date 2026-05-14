//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 688/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk688<F: Float>(t11644: F, t11656: F, t11780: F, t11799: F, t103: F, t82: F, t3205: F, t8372: F, t100: F, t1587: F) -> (F, F, F, F) {
    let t11801 = t11644 + t11656 + t11780 + t11799;
    let t11803 = t82 * t11801 * t103;
    let t11807 = t8372 * t3205;
    let t11810 = t1587 * t100;
    (t11801, t11803, t11807, t11810)
}
