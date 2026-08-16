//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 359/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk359(t2085: f64, t648: f64, t2028: f64, t305: f64, t2031: f64, t326: f64, t118: f64, t2025: f64, t2069: f64, t793: f64, t2074: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2086 = t648 * t2085;
    let t2087 = 0.90915538847484472429e-2_f64 * t2086;
    let t2088 = t305 * t2028;
    let t2090 = t326 * t2031;
    let t2092 = t118 * t2025;
    let t2094 = t793 * t2069;
    let t2096 = t797 * t2074;
    (t2087, t2088, t2090, t2092, t2094, t2096)
}
