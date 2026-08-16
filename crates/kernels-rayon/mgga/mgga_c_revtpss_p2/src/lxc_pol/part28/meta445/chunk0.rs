//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1679/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1679(t1169: f64, t17085: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64, t3386: f64, t1749: f64, t3520: f64, t16868: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16871: f64, t16876: f64) -> (f64, f64, f64, f64, f64) {
    let t17086 = t17085 * t1169;
    let t17089 = t5155 * t1179;
    let t17092 = t1719 * t3383;
    let t17094 = 2.0_f64 * t17092 * t3386;
    let t17097 = t1749 * t3520;
    let t17115 = 0.11038e0_f64 * t16868;
    let t17117 = 0.20128333333333333334e0_f64 * t16712;
    let t17126 = -t17115 + 0.82785e-1_f64 * t16871 - t17117 + 0.301925e0_f64 * t16748 + 0.13418888888888888889e0_f64 * t16706 + 0.91983333333333333334e-1_f64 * t16876 + 0.67094444444444444447e-1_f64 * t12299 + 0.26837777777777777778e0_f64 * t12297 - 0.20128333333333333334e0_f64 * t12301 - 0.10064166666666666667e0_f64 * t12303 - 0.40256666666666666666e0_f64 * t16727;
    (t17086, t17089, t17094, t17097, t17126)
}
