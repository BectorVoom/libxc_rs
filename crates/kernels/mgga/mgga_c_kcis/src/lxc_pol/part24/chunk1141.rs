//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1141/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1141<F: Float>(t26464: F, t2726: F, t8764: F, t882: F, t26463: F, t213: F, t2751: F, t6: F, t887: F, t26470: F, t26465: F, t2746: F, t8525: F) -> (F, F, F, F, F, F, F, F) {
    let t91966 = t26464 * t8764 * t2726 * t882;
    let t91967 = t26463 * t91966;
    let t91972 = t6 * t213 * t8764 * t887 * t2751;
    let t91973 = t26463 * t91972;
    let t91975 = t26470 * t91966;
    let t91978 = t26464 * t26465 * t2751;
    let t91979 = t26463 * t91978;
    let t91982 = t26464 * t8525 * t2746;
    (t91966, t91967, t91972, t91973, t91975, t91978, t91979, t91982)
}
