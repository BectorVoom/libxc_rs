//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 960/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk960(t1580: f64, t779: f64, t689: f64, t1579: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64) {
    let t4477 = t779 * t1580;
    let t4478 = t689 * t4477;
    let t4480 = t1579 * t72;
    let t4481 = t4480 * t686;
    (t4477, t4478, t4480, t4481)
}
