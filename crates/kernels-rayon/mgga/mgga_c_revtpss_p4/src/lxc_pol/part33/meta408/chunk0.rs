//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1459/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1459(t1811: f64, t3555: f64, t460: f64, t5412: f64, t17306: f64, t487: f64, t1269: f64, t5219: f64, t5216: f64, t1204: f64, t1209: f64, t17288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18037 = t3555 * t1811;
    let t18054 = t460 * t5412;
    let t18059 = t17306 * t487;
    let t18062 = t5219 * t1269;
    let t18065 = t5216 * t487;
    let t18087 = t1204 * t1811;
    let t18097 = t1209 * t5412;
    let t18114 = t17288 * t487;
    (t18037, t18054, t18059, t18062, t18065, t18087, t18097, t18114)
}
