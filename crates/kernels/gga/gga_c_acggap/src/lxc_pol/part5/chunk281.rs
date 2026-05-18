//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 281/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk281<F: Float>(t22: F, t594: F, t161: F, t151: F, t177: F, t377: F, t414: F, t150: F, t848: F) -> (F, F, F, F, F, F, F) {
    let t972 = F::new(1.0) / t22 / t594;
    let t973 = t161 * t972;
    let t974 = t151 * t973;
    let t976 = F::new(0.56688979511669985553e-2) * t974 * t177;
    let t977 = t377 * t414;
    let t979 = F::new(0.20007875121765877254e-2) * t977 * t177;
    let t980 = t848 * t150;
    (t972, t973, t974, t976, t977, t979, t980)
}
