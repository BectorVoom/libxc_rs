//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1171/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1171(t1072: f64, t31137: f64, t513: f64, t721: f64, t2019: f64, t2029: f64, t8807: f64, t31142: f64, t8810: f64, t1314: f64, t361: f64, t8806: f64) -> (f64, f64, f64, f64) {
    let t36036 = t31137 * t1072 * t513 * t721;
    let t36039 = t2019 * t2029 * t8807;
    let t36040 = 7.0_f64 / 24.0_f64 * t36039;
    let t36041 = t31142 * t8810;
    let t36042 = 7.0_f64 / 72.0_f64 * t36041;
    let t36044 = t8806 * t361 * t1314;
    (t36036, t36040, t36042, t36044)
}
