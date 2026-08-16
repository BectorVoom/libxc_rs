//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 693/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk693(t4306: f64, t190: f64, t4186: f64, t706: f64, t1531: f64, t705: f64) -> (f64, f64, f64, f64) {
    let t4307 = 4.0_f64 * t4306;
    let t4308 = t190 * t4186;
    let t4310 = 4.0_f64 * t706 * t4308;
    let t4311 = t705 * t1531;
    (t4307, t4308, t4310, t4311)
}
