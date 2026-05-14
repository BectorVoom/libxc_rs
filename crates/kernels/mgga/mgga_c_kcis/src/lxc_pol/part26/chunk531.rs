//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 531/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk531<F: Float>(t5427: F, t5701: F, t1380: F, t1889: F, t3984: F, t1370: F, t498: F) -> (F, F, F, F) {
    let t5702 = t5701 * t5427;
    let t5705 = t1889 * t1380;
    let t5706 = t3984 * t5705;
    let t5709 = t1370 * t498;
    (t5702, t5705, t5706, t5709)
}
