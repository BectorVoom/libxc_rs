//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 232/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk232<F: Float>(t410: F, t417: F, t971: F, t431: F, t176: F, t409: F, t416: F) -> (F, F, F, F) {
    let t973 = t410 * t971 * t417;
    let t975 = F::new(0.5848223622634646207e0) * t431 * t973;
    let t976 = t409 * t176;
    let t977 = F::new(1.0) / t976;
    let t978 = t416 * t416;
    (t973, t975, t977, t978)
}
