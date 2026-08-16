//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 871/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk871(t2158: f64, t798: f64, t291: f64, t653: f64, t2418: f64, t297: f64, t2165: f64, t3247: f64, t2300: f64, t3188: f64, t3187: f64, t2885: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t10122 = t2158 * t798;
    let t10123 = t653 * t291;
    let t10125 = t10123 * t297 * t2418;
    let t10126 = t10122 * t10125;
    let t10128 = t2165 * t3247;
    let t10130 = t3188 * t2300;
    let t10131 = t3187 * t10130;
    let t10133 = t2885 * t820;
    (t10123, t10126, t10128, t10131, t10133)
}
