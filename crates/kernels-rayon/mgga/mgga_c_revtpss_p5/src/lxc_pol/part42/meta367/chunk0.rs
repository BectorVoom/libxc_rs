//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1186/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1186(t670: f64, t6765: f64, t1843: f64, t4292: f64, t1310: f64, t5920: f64, t116: f64, t5876: f64) -> (f64, f64, f64, f64) {
    let t18232 = t6765 * t670;
    let t18235 = t1843 * t4292;
    let t18242 = t1310 * t5920;
    let t18245 = t5876 * t116;
    (t18232, t18235, t18242, t18245)
}
