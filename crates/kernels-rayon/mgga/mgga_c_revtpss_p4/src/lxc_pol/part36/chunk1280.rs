//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1280/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1280(t213: f64, t30055: f64, t689: f64, t6896: f64, t7242: f64, t22399: f64, t26054: f64, t27888: f64, t27899: f64, t27884: f64, t27873: f64, t97700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108395 = t213 * t30055;
    let t108411 = t689 * t7242 * t6896;
    let t108422 = t26054 * t22399;
    let t108431 = t27899 * t27888;
    let t108435 = t27884 * t27888;
    let t108438 = t97700 * t27873;
    (t108395, t108411, t108422, t108431, t108435, t108438)
}
