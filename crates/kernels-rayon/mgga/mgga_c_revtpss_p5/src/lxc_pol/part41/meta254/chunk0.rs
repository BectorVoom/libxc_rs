//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 971/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk971(t1453: f64, t2178: f64, t1312: f64, t2179: f64, t2181: f64, t2322: f64, t4254: f64, t5523: f64, t651: f64, t8254: f64, t8274: f64, t8278: f64) -> (f64, f64) {
    let t8280 = t2178 * t1453;
    let t8283 = 2.0_f64 * t1312 * t8278 + 2.0_f64 * t1312 * t8280 - 2.0_f64 * t2179 * t2322 - 2.0_f64 * t2179 * t4254 + 2.0_f64 * t2181 * t2322 + 2.0_f64 * t2181 * t5523 - 2.0_f64 * t651 * t8254 - 2.0_f64 * t651 * t8274;
    (t8280, t8283)
}
