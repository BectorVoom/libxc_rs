//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 272/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk272<F: Float>(t934: F, t935: F, t915: F, t902: F, t908: F, t307: F, t302: F, t928: F, t919: F, t924: F, t932: F, t310: F) -> (F, F, F, F, F, F, F, F) {
    let t936 = t934 * t935;
    let t938 = F::new(1.0) * t915 * t936;
    let t939 = F::new(0.17123333333333333333e-1) * t902;
    let t941 = -t939 - F::new(0.17123333333333333333e-1) * t908;
    let t944 = t307 * t307;
    let t945 = F::new(1.0) / t944;
    let t946 = t302 * t945;
    let t948 = F::new(0.516475e0) * t902;
    let t951 = F::new(0.104195e0) * t928;
    let t953 = F::new(0.3529725e1) * t919 - t948 - F::new(0.516475e0) * t908 + F::new(0.6311625e0) * t924 - t951 - F::new(0.104195e0) * t932;
    let t954 = F::new(1.0) / t310;
    (t936, t938, t941, t944, t945, t946, t953, t954)
}
