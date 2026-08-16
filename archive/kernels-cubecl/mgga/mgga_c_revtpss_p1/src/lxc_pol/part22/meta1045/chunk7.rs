//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3666/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3666<F: Float>(t43911: F, t56176: F, t56183: F, t56185: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t68363: F, t68366: F, t68368: F, t68370: F, t68373: F) -> F {
    let t69263 = F::cast_from(0.57386111111111111112e0_f64) * t68342 + F::cast_from(0.68863333333333333334e1_f64) * t68347 - F::cast_from(0.20659e1_f64) * t68350 - F::cast_from(0.123954e2_f64) * t68353 - F::cast_from(0.68863333333333333334e0_f64) * t68357 + F::cast_from(0.123954e2_f64) * t68360 - F::cast_from(0.82636000000000000001e1_f64) * t68363 + F::cast_from(0.22954444444444444444e1_f64) * t68366 - F::cast_from(0.27785333333333333334e0_f64) * t68368 - F::cast_from(0.61745185185185185186e-1_f64) * t68370 + F::cast_from(0.6311625e0_f64) * t68373 - F::cast_from(0.3859074074074074074e-1_f64) * t43911 - F::cast_from(0.6121185185185185185e0_f64) * t56176 + F::cast_from(0.18363555555555555555e1_f64) * t56183 - F::cast_from(0.13772666666666666666e1_f64) * t56185;
    t69263
}
