//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 824/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk824<F: Float>(t274: F, t4977: F, t21130: F, t683: F, t1095: F, t231: F, t10327: F, t992: F, t19168: F, t801: F, t278: F, t1193: F) -> (F, F, F, F, F, F, F, F) {
    let t22096 = t274 * t4977;
    let t22100 = t683 * t21130 * t274;
    let t22107 = t231 * t4977 * t1095 * t274;
    let t22110 = t10327 * t992;
    let t22111 = t19168 * t22110;
    let t22116 = t231 * t21130 * t801 * t274;
    let t22119 = t21130 * t278;
    let t22122 = t1193 * t4977;
    (t22096, t22100, t22107, t22110, t22111, t22116, t22119, t22122)
}
