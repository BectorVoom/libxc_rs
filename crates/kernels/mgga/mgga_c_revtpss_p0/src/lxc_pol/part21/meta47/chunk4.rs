//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 353/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk353<F: Float>(t902: F, t928: F, t908: F, t919: F, t924: F, t932: F) -> (F, F, F) {
    let t948 = F::cast_from(0.516475e0_f64) * t902;
    let t951 = F::cast_from(0.104195e0_f64) * t928;
    let t953 = F::cast_from(0.3529725e1_f64) * t919 - t948 - F::cast_from(0.516475e0_f64) * t908 + F::cast_from(0.6311625e0_f64) * t924 - t951 - F::cast_from(0.104195e0_f64) * t932;
    (t948, t951, t953)
}
