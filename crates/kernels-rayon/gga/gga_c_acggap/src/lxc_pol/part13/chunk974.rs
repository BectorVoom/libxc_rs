//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 974/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk974(t2132: f64, t2138: f64, t3101: f64, t609: f64, t7941: f64, t862: f64, t32004: f64, t7965: f64, t2131: f64, t2147: f64, t463: f64, t7997: f64) -> (f64, f64, f64, f64) {
    let t32180 = 0.8673628188205199462e0_f64 * t2138 * t2132 * t609 * t3101;
    let t32181 = t862 * t7941;
    let t32183 = t32181 * t32004 * t7965;
    let t32187 = t2131 * t2147 * t7997 * t463;
    (t32180, t32181, t32183, t32187)
}
