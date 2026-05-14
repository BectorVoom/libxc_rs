//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 165/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk165<F: Float>(t792: F, t992: F, t666: F, t89: F, t1095: F, t801: F, t278: F, t274: F, t807: F) -> (F, F, F, F, F) {
    let t1186 = t792 * t992;
    let t1188 = t89 * t666 * t1186;
    let t1190 = t801 * t1095;
    let t1193 = t1095 * t278;
    let t1196 = 0.23410285231011484e0 * t1190 * t274 - 0.532971647967385935e-1 * t807 * t1193;
    (t1186, t1188, t1190, t1193, t1196)
}
