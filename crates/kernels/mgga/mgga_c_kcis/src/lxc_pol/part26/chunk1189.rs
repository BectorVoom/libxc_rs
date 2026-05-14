//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1189/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1189<F: Float>(t29487: F, t4184: F, t7271: F, t94816: F, t2069: F, t99724: F, t54732: F, t7943: F, t12338: F, t29427: F, t2253: F, t54773: F, t28644: F, t5897: F, t22313: F, t27494: F) -> (F, F, F, F, F, F, F, F) {
    let t102842 = t4184 * t29487;
    let t102846 = 2.0 * t94816 * t7271;
    let t102848 = 2.0 * t99724 * t2069;
    let t102850 = 2.0 * t54732 * t7943;
    let t102854 = 4.0 * t12338 * t29427;
    let t102855 = t54773 * t2253;
    let t102860 = 2.0 * t5897 * t28644;
    let t102864 = 4.0 * t27494 * t22313;
    (t102842, t102846, t102848, t102850, t102854, t102855, t102860, t102864)
}
