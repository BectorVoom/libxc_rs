//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 943/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk943(t1049: f64, t2954: f64, t2953: f64, t417: f64, t412: f64, t9181: f64, t9213: f64, t1052: f64, t2956: f64, t420: f64, t2929: f64, t1022: f64, t2909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9419 = t1049 * t2954;
    let t9423 = 1.0_f64 / t2953 / t417;
    let t9424 = t412 * t9423;
    let t9429 = 0.46308888888888888888e0_f64 * t9181;
    let t9438 = 0.16068111111111111111e1_f64 * t9213;
    let t9464 = 1.0_f64 / t2953 / t1052;
    let t9465 = t412 * t9464;
    let t9467 = 1.0_f64 / t2956 / t420;
    let t9471 = t1049 * t2929;
    let t9477 = 0.53272592592592592592e-1_f64 * t9213;
    let t9492 = 1.0_f64 / t2909 / t1022;
    (t9419, t9424, t9429, t9438, t9465, t9467, t9471, t9477, t9492)
}
