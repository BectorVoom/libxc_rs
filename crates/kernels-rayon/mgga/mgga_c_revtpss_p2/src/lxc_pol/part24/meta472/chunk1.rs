//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1453/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1453(t18729: f64, t2470: f64, t2798: f64, t2482: f64, t6016: f64, t879: f64, t14563: f64, t14568: f64, t10535: f64, t136: f64, t2457: f64, t6017: f64) -> (f64, f64, f64, f64) {
    let t62952 = t2798 * t18729 * t2470;
    let t62967 = t2482 * t879 * t6016;
    let t62983 = t14568 * t14563;
    let t62999 = t10535 * t6017 * t136 * t2457;
    (t62952, t62967, t62983, t62999)
}
