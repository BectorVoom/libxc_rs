//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2085/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2085(t12167: f64, t99984: f64, t12078: f64, t25516: f64, t4954: f64, t15752: f64, t27498: f64, t15734: f64, t25522: f64, t15816: f64, t7121: f64, t15794: f64, t25580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100138 = t12167 * t99984;
    let t100141 = t12078 * t99984;
    let t100146 = t4954 * t25516;
    let t100160 = 0.57165357490759649296e-3_f64 * t27498 * t15752;
    let t100166 = t25522 * t15734;
    let t100168 = t15816 * t7121;
    let t100186 = 0.57165357490759649296e-3_f64 * t25580 * t15794;
    (t100138, t100141, t100146, t100160, t100166, t100168, t100186)
}
