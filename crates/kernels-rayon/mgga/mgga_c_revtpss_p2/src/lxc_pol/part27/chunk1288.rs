//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1288/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1288(t3813: f64, t651: f64, t7002: f64, t18163: f64, t7003: f64, t25861: f64, t4254: f64, t25188: f64, t7316: f64, t10426: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64) {
    let t95011 = 6.0_f64 * t651 * t3813 * t7002;
    let t95013 = 6.0_f64 * t18163 * t7003;
    let t95015 = 12.0_f64 * t4254 * t25861;
    let t95017 = 3.0_f64 * t25188 * t7316;
    let t95019 = t10426 * t196 * t197;
    (t95011, t95013, t95015, t95017, t95019)
}
