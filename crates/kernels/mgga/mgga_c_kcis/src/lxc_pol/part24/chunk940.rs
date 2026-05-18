//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 940/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk940<F: Float>(t19956: F, t376: F, t375: F, t19619: F, t5176: F, t5175: F, t5068: F, t5172: F, t1166: F, t6701: F, t1817: F, t5169: F) -> (F, F, F, F, F, F) {
    let t19957 = t376 * t19956;
    let t19958 = t375 * t19957;
    let t19960 = t5176 * t19619;
    let t19961 = t5175 * t19960;
    let t19963 = t5172 * t5068;
    let t19965 = t1166 * t6701;
    let t19967 = t5169 * t1817;
    (t19958, t19960, t19961, t19963, t19965, t19967)
}
