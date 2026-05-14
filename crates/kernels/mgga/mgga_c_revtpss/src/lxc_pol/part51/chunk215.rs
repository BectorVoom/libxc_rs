//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 215/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk215<F: Float>(t141: F, t931: F, t908: F, t919: F, t921: F, t924: F, t929: F, t290: F, t915: F, t902: F, t307: F, t302: F, t928: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t932 = t141 * t931;
    let t934 = 0.1898925e1 * t919 - t921 - 0.29896666666666666667e0 * t908 + 0.3071625e0 * t924 - t929 - 0.82156666666666666667e-1 * t932;
    let t935 = 1.0 / t290;
    let t936 = t934 * t935;
    let t938 = 1.0 * t915 * t936;
    let t939 = 0.17123333333333333333e-1 * t902;
    let t941 = -t939 - 0.17123333333333333333e-1 * t908;
    let t944 = t307 * t307;
    let t945 = 1.0 / t944;
    let t946 = t302 * t945;
    let t948 = 0.516475e0 * t902;
    let t951 = 0.104195e0 * t928;
    let t953 = 0.3529725e1 * t919 - t948 - 0.516475e0 * t908 + 0.6311625e0 * t924 - t951 - 0.104195e0 * t932;
    let t954 = 1.0 / t310;
    let t955 = t953 * t954;
    let t958 = 0.92708333333333333333e-2 * t902;
    (t932, t934, t935, t936, t938, t939, t941, t944, t945, t946, t948, t951, t953, t954, t955, t958)
}
