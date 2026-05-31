//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 707/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk707<F: Float>(t3437: F, t548: F, t1529: F, t822: F, t1982: F, t515: F, t1960: F, t568: F, t3380: F, t3385: F, t3388: F, t3391: F) -> (F, F, F, F, F, F, F, F) {
    let t4464 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t548 * t3437;
    let t4465 = t822 * t1529;
    let t4466 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t4465;
    let t4468 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1982 * t515;
    let t4470 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1960 * t568;
    let t4471 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3380;
    let t4472 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3385;
    let t4473 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3388;
    let t4474 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3391;
    (t4464, t4466, t4468, t4470, t4471, t4472, t4473, t4474)
}
