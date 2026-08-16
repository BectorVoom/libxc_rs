//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1079/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1079(t1235: f64, t20842: f64, t127: f64, t371: f64, t6609: f64, t3671: f64, t1208: f64, t6563: f64) -> (f64, f64, f64, f64) {
    let t20843 = t1235 * t20842;
    let t20846 = t371 * t127 * t6609;
    let t20847 = t3671 * t20846;
    let t20849 = t6563 * t1208;
    (t20843, t20846, t20847, t20849)
}
