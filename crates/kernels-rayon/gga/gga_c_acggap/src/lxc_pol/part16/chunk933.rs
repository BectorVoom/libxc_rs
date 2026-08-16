//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 933/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk933(t32165: f64, t7998: f64, t7987: f64, t2131: f64, t2132: f64, t3644: f64, t609: f64, t2138: f64, t3101: f64, t7941: f64, t862: f64, t32004: f64, t7965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32167 = 0.26020884564615598386e1_f64 * t32165 * t7998;
    let t32168 = t7987 * t7998;
    let t32176 = 0.8673628188205199462e0_f64 * t2131 * t2132 * t609 * t3644;
    let t32180 = 0.8673628188205199462e0_f64 * t2138 * t2132 * t609 * t3101;
    let t32181 = t862 * t7941;
    let t32183 = t32181 * t32004 * t7965;
    (t32167, t32168, t32176, t32180, t32181, t32183)
}
