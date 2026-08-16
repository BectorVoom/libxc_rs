//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2094/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2094(t29010: f64, t3704: f64, t17720: f64, t7624: f64, t15904: f64, t26865: f64, t13127: f64, t17400: f64, t26866: f64, t1802: f64, t3089: f64, t3717: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t104689 = 0.57165357490759649296e-3_f64 * t29010 * t3704;
    let t104691 = 0.6351706387862183255e-3_f64 * t7624 * t17720;
    let t104695 = t26865 * t15904;
    let t104696 = t13127 * t104695;
    let t104703 = t17400 * t26866;
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    (t104689, t104691, t104695, t104696, t104703, t104706, t104707, t104708)
}
