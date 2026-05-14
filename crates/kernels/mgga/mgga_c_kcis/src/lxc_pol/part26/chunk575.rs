//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 575/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk575<F: Float>(t5427: F, t6151: F, t1610: F, t1889: F, t4440: F, t1370: F, t617: F) -> (F, F, F, F) {
    let t6152 = t6151 * t5427;
    let t6155 = t1889 * t1610;
    let t6156 = t4440 * t6155;
    let t6159 = t1370 * t617;
    (t6152, t6155, t6156, t6159)
}
