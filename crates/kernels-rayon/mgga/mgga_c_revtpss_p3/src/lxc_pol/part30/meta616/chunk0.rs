//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2122/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2122(t27123: f64, t7003: f64, t13514: f64, t94: f64, t1937: f64, t27126: f64, t6993: f64, t25178: f64, t7898: f64, t22496: f64, t25082: f64, t32113: f64) -> (f64, f64, f64, f64, f64) {
    let t98534 = 4.0_f64 * t27123 * t7003;
    let t98535 = t94 * t13514;
    let t98537 = 2.0_f64 * t98535 * t1937;
    let t98539 = 4.0_f64 * t27126 * t6993;
    let t98541 = 2.0_f64 * t7898 * t25178;
    let t98544 = 6.0_f64 * t25082 * t32113 * t22496;
    (t98534, t98537, t98539, t98541, t98544)
}
