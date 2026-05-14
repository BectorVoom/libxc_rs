//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 830/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk830<F: Float>(t2099: F, t4425: F, t1599: F, t3978: F, t617: F, t5427: F, t1610: F, t1889: F, t4440: F, t1370: F, t5441: F, t1369: F, t737: F, t1601: F, t167: F, t2105: F, t25: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6148 = t4425 * t2099;
    let t6149 = t1599 * t6148;
    let t6151 = t3978 * t617;
    let t6152 = t6151 * t5427;
    let t6155 = t1889 * t1610;
    let t6156 = t4440 * t6155;
    let t6159 = t1370 * t617;
    let t6160 = t6159 * t5441;
    let t6163 = t737 * t1369;
    let t6164 = t1601 * t167;
    let t6165 = t6163 * t6164;
    let t6168 = t25 * t2105;
    (t6149, t6151, t6152, t6155, t6156, t6159, t6160, t6164, t6165, t6168)
}
