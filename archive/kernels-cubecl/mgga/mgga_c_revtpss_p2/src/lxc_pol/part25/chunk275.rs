//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 275/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk275<F: Float>(t934: F, t935: F, t915: F, t902: F, t908: F, t307: F, t302: F, t928: F, t919: F, t924: F, t932: F, t310: F) -> (F, F, F, F, F, F, F, F) {
    let t936 = t934 * t935;
    let t938 = F::cast_from(1.0_f64) * t915 * t936;
    let t939 = F::cast_from(0.17123333333333333333e-1_f64) * t902;
    let t941 = -t939 - F::cast_from(0.17123333333333333333e-1_f64) * t908;
    let t944 = t307 * t307;
    let t945 = F::cast_from(1.0_f64) / t944;
    let t946 = t302 * t945;
    let t948 = F::cast_from(0.516475e0_f64) * t902;
    let t951 = F::cast_from(0.104195e0_f64) * t928;
    let t953 = F::cast_from(0.3529725e1_f64) * t919 - t948 - F::cast_from(0.516475e0_f64) * t908 + F::cast_from(0.6311625e0_f64) * t924 - t951 - F::cast_from(0.104195e0_f64) * t932;
    let t954 = F::cast_from(1.0_f64) / t310;
    (t936, t938, t941, t944, t945, t946, t953, t954)
}
