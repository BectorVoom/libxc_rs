//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1467/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1467(t31032: f64, t8269: f64, t10208: f64, t69: f64, t101: f64, t43: f64, t100: f64, t2349: f64, t96: f64, t116: f64, t8273: f64, t1453: f64, t8362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31033 = t31032 * t8269;
    let t31035 = t69 * t10208;
    let t31039 = t43 * t101;
    let t31054 = t43 * t100;
    let t31058 = t96 * t2349;
    let t31117 = t116 * t8273;
    let t31248 = t8362 * t1453;
    (t31033, t31035, t31039, t31054, t31058, t31117, t31248)
}
