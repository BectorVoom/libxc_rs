//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 724/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk724(t4185: f64, t4188: f64, t4190: f64, t4193: f64, t4198: f64, t4201: f64, t4202: f64, t4544: f64, t4547: f64, t4549: f64, t4550: f64, t4551: f64, t4552: f64, t4553: f64, t4554: f64, t4555: f64) -> f64 {
    let t4560 = 0.07214027574909895_f64 * t4544 + 0.011181742741110338_f64 * t4547 + t4549 - t4550 - t4551 + t4552 - t4553 - t4554 - t4555 - t4185 + 0.10821041362364843_f64 * t4188 + 0.4328416544945937_f64 * t4190 + 0.022363485482220676_f64 * t4193 + t4198 + t4201 + 0.1442805514981979_f64 * t4202;
    t4560
}
