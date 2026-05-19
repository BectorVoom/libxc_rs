//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 724/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk724<F: Float>(t4185: F, t4188: F, t4190: F, t4193: F, t4198: F, t4201: F, t4202: F, t4544: F, t4547: F, t4549: F, t4550: F, t4551: F, t4552: F, t4553: F, t4554: F, t4555: F) -> F {
    let t4560 = F::cast_from(0.07214027574909895_f64) * t4544 + F::cast_from(0.011181742741110338_f64) * t4547 + t4549 - t4550 - t4551 + t4552 - t4553 - t4554 - t4555 - t4185 + F::cast_from(0.10821041362364843_f64) * t4188 + F::cast_from(0.4328416544945937_f64) * t4190 + F::cast_from(0.022363485482220676_f64) * t4193 + t4198 + t4201 + F::cast_from(0.1442805514981979_f64) * t4202;
    t4560
}
