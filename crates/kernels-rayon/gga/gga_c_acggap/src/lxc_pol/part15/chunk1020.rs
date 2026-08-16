//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1020/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1020(t7433: f64, t8908: f64, t8912: f64, t7346: f64, t7347: f64, t8480: f64, t7447: f64, t8823: f64, t7440: f64, t8826: f64, t30817: f64, t8948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35835 = t7433 * t8908;
    let t35837 = t7433 * t8912;
    let t35844 = t7346 * t8480 * t7347;
    let t35848 = t7447 * t8823;
    let t35850 = t7440 * t8826;
    let t35874 = t30817 * t8948;
    (t35835, t35837, t35844, t35848, t35850, t35874)
}
