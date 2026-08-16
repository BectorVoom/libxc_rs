//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1951/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951(t1923: f64, t26204: f64, t7719: f64, t101214: f64, t2047: f64, t101218: f64, t101237: f64, t101240: f64, t101243: f64, t101303: f64, t101376: f64, t2048: f64, t25117: f64, t25162: f64, t26182: f64, t28154: f64, t28628: f64, t28635: f64, t6954: f64, t7964: f64, t92588: f64, t95303: f64) -> f64 {
    let t101929 = t1923 * t26204 * t7719;
    let t101935 = t2047 * t101214;
    let t101938 = t2047 * t101218;
    let t101949 = 2.0_f64 / 3.0_f64 * t6954 * t28635 + t1923 * t2047 * t101303 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t101376 * t2048 + 88.0_f64 / 27.0_f64 * t101929 - 2.0_f64 / 3.0_f64 * t25117 * t7964 + 10.0_f64 / 3.0_f64 * t92588 * t28628 + 20.0_f64 / 3.0_f64 * t25162 * t101935 + 20.0_f64 / 3.0_f64 * t25162 * t101938 + 20.0_f64 / 3.0_f64 * t101237 * t26182 + 20.0_f64 / 3.0_f64 * t101240 * t26182 + 20.0_f64 / 3.0_f64 * t101243 * t26182 + 20.0_f64 / 3.0_f64 * t28154 * t95303;
    t101949
}
