//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 666/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk666(t104: f64, t2162: f64, t566: f64, t95: f64, t2133: f64, t463: f64, t2147: f64, t2131: f64, t130: f64, t595: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7292 = t104 * t2162;
    let t7297 = t566 * t95 * t104;
    let t7305 = t2133 * t463;
    let t7306 = t2147 * t7305;
    let t7307 = t2131 * t7306;
    let t7309 = t130 * t595;
    let t7310 = t7309 * t154;
    (t7292, t7297, t7306, t7307, t7309, t7310)
}
