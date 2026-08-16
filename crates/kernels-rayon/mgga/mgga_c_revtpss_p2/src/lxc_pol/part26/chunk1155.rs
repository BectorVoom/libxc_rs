//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1155/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1155(t25081: f64, t7234: f64, t1464: f64, t7541: f64, t26703: f64, t575: f64, t26743: f64, t571: f64, t1455: f64, t7560: f64, t2110: f64, t4168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95088 = t7234 * t25081;
    let t95182 = t7541 * t1464;
    let t95184 = t26703 * t575;
    let t95186 = t571 * t26743;
    let t95190 = t1455 * t7560;
    let t95196 = t2110 * t4168;
    (t95088, t95182, t95184, t95186, t95190, t95196)
}
