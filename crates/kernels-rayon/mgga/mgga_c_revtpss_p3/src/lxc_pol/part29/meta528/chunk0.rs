//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1856/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1856(t26703: f64, t575: f64, t26743: f64, t571: f64, t1455: f64, t7560: f64, t2110: f64, t4168: f64, t1923: f64, t25146: f64, t7348: f64, t25150: f64, t7349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95184 = t26703 * t575;
    let t95186 = t571 * t26743;
    let t95190 = t1455 * t7560;
    let t95196 = t2110 * t4168;
    let t95230 = t1923 * t7348 * t25146;
    let t95241 = t25150 * t7349;
    (t95184, t95186, t95190, t95196, t95230, t95241)
}
