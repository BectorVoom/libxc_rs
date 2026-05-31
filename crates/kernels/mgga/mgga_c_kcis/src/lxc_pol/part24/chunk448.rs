//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 448/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk448<F: Float>(t312: F, t2943: F, t308: F, t1042: F, t932: F, t2917: F, t1075: F, t317: F, t319: F, t2469: F, t251: F, t323: F, t325: F) -> (F, F, F, F, F, F, F, F) {
    let t3072 = t312 * t312;
    let t3073 = F::cast_from(1.0_f64) / t3072;
    let t3078 = t2943 * t308;
    let t3081 = t932 * t1042;
    let t3088 = F::cast_from(0.55033333333333333333e-2_f64) * t2917;
    let t3105 = F::cast_from(0.8197e-2_f64) * t317 * t1075 * t319;
    let t3106 = t2469 * t251;
    let t3109 = F::cast_from(0.21133333333333333333e-2_f64) * t323 * t3106 * t325;
    (t3072, t3073, t3078, t3081, t3088, t3105, t3106, t3109)
}
