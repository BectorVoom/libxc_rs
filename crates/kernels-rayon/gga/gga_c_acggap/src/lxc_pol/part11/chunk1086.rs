//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1086/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1086(t174: f64, t7815: f64, t4257: f64, t7450: f64, t301: f64, t4256: f64, t8539: f64, t2030: f64, t372: f64, t4262: f64, t17752: f64, t8923: f64) -> (f64, f64, f64, f64) {
    let t34903 = t7815 * t174;
    let t34905 = t7450 * t34903 * t4257;
    let t34909 = t7450 * t4256 * t8539 * t301;
    let t34913 = t2030 * t4262 * t8539 * t372;
    let t34916 = t2030 * t17752 * t8923;
    (t34905, t34909, t34913, t34916)
}
