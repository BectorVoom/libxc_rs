//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 985/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk985(t1139: f64, t24312: f64, t1132: f64, t1723: f64, t6442: f64, t12327: f64, t12331: f64, t12349: f64, t12352: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64) -> (f64, f64, f64, f64, f64) {
    let t24313 = t1139 * t24312;
    let t24315 = t1132 * t24312;
    let t24317 = t6442 * t1723;
    let t24318 = t12327 * t24317;
    let t24320 = t12331 * t24317;
    let t24322 = 0.17938e1_f64 * t24242 + 0.29896666666666666667e0_f64 * t24250 - 0.16431333333333333333e0_f64 * t24289 + 0.49293999999999999999e0_f64 * t24292 + 0.82156666666666666667e-1_f64 * t24295 - t12349 - t12352 - 0.82156666666666666668e-1_f64 * t24298 - 0.59793333333333333333e0_f64 * t24238 + 0.17938e1_f64 * t24246 + 0.3071625e0_f64 * t24313 + 0.1898925e1_f64 * t24315 + 0.142419375e1_f64 * t24318 - 0.76790625e-1_f64 * t24320;
    (t24313, t24315, t24318, t24320, t24322)
}
