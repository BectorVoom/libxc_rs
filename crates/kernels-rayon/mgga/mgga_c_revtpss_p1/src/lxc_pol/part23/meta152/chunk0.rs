//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 942/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk942(t2662: f64, t4353: f64, t2661: f64, t1565: f64, t2652: f64, t1561: f64, t2741: f64, t241: f64, t2719: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t4354 = t2662 * t4353;
    let t4355 = t2661 * t4354;
    let t4357 = t2652 * t1565;
    let t4359 = t2741 * t1561;
    let t4362 = t820 * t2719 * t241;
    (t4354, t4355, t4357, t4359, t4362)
}
