//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 642/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk642<F: Float>(t3304: F, t3594: F, t2608: F, t3308: F, t574: F, t1055: F, t980: F, t1060: F, t938: F) -> (F, F, F, F, F) {
    let t3595 = t3304 * t3594;
    let t3597 = t3308 * t2608;
    let t3598 = t574 * t3597;
    let t3600 = t980 * t1055;
    let t3602 = t1060 * t938;
    (t3595, t3597, t3598, t3600, t3602)
}
