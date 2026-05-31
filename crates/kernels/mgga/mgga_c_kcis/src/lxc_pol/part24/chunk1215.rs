//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1215/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1215<F: Float>(t19839: F, t26871: F, t10491: F, t29039: F, t14668: F, t28009: F, t20210: F, t2189: F, t3330: F, t28005: F, t10498: F, t1203: F) -> (F, F, F, F, F, F) {
    let t99852 = F::cast_from(4.0_f64) * t26871 * t19839;
    let t99854 = F::cast_from(4.0_f64) * t10491 * t29039;
    let t99856 = F::cast_from(4.0_f64) * t14668 * t28009;
    let t99859 = F::cast_from(2.0_f64) * t3330 * t2189 * t20210;
    let t99861 = F::cast_from(4.0_f64) * t14668 * t28005;
    let t99864 = F::cast_from(12.0_f64) * t10498 * t29039 * t1203;
    (t99852, t99854, t99856, t99859, t99861, t99864)
}
