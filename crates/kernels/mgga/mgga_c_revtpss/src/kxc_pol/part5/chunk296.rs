//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 296/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk296<F: Float>(t902: F, t928: F, t908: F, t919: F, t924: F, t932: F) -> (F, F, F) {
    let t967 = F::cast_from(0.301925e0_f64) * t902;
    let t970 = F::cast_from(0.82785e-1_f64) * t928;
    let t972 = F::cast_from(0.258925e1_f64) * t919 - t967 - F::cast_from(0.301925e0_f64) * t908 + F::cast_from(0.16504875e0_f64) * t924 - t970 - F::cast_from(0.82785e-1_f64) * t932;
    (t967, t970, t972)
}
