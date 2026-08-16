//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 220/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk220(t934: f64, t935: f64, t915: f64, t902: f64, t908: f64, t307: f64, t302: f64, t928: f64, t919: f64, t924: f64, t932: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t936 = t934 * t935;
    let t938 = 1.0_f64 * t915 * t936;
    let t939 = 0.17123333333333333333e-1_f64 * t902;
    let t941 = -t939 - 0.17123333333333333333e-1_f64 * t908;
    let t944 = t307 * t307;
    let t945 = 1.0_f64 / t944;
    let t946 = t302 * t945;
    let t948 = 0.516475e0_f64 * t902;
    let t951 = 0.104195e0_f64 * t928;
    let t953 = 0.3529725e1_f64 * t919 - t948 - 0.516475e0_f64 * t908 + 0.6311625e0_f64 * t924 - t951 - 0.104195e0_f64 * t932;
    let t954 = 1.0_f64 / t310;
    (t936, t938, t939, t941, t944, t945, t946, t948, t951, t953, t954)
}
