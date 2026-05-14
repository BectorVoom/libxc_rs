//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1182/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1182<F: Float>(t24378: F, t25077: F, t28548: F, t109015: F, t28603: F, t28562: F, t28574: F, t28579: F, t24330: F, t28583: F, t6242: F, t6249: F, t7012: F, t96535: F, t54863: F, t7005: F) -> (F, F, F, F, F, F, F) {
    let t111861 = 0.22226000364197530866e-1 * t25077 * t24378 * t28548;
    let t111868 = 0.26853068634149852184e-1 * t28603 * t109015;
    let t111881 = t25077 * t24378 * t28562;
    let t111889 = t28579 * t28574;
    let t111892 = t6242 * t24330 * t28583;
    let t111895 = t6249 * t96535 * t7012;
    let t111897 = t54863 * t7005;
    (t111861, t111868, t111881, t111889, t111892, t111895, t111897)
}
