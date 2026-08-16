//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3250/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250(t1358: f64, t212: f64, t22964: f64, t689: f64, t13848: f64, t22893: f64, t47274: f64, t9816: f64, t22890: f64, t9962: f64, t13845: f64, t22841: f64, t73731: f64, t9818: f64) -> (f64, f64, f64, f64) {
    let t85509 = t689 * t212 * t22964 * t1358;
    let t85514 = t9816 * t47274 * t13848 * t22893;
    let t85516 = t9962 * t22890;
    let t85532 = t13845 * t9818 * t73731 * t22841;
    (t85509, t85514, t85516, t85532)
}
