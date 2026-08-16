//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 668/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk668(t469: f64, t922: f64, t104: f64, t2162: f64, t566: f64, t95: f64, t2541: f64, t3984: f64, t839: f64, t2133: f64, t463: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7288 = t469 * t922;
    let t7292 = t104 * t2162;
    let t7297 = t566 * t95 * t104;
    let t7298 = t2541 * t3984;
    let t7301 = t469 * t839;
    let t7305 = t2133 * t463;
    let t7306 = t2147 * t7305;
    (t7288, t7292, t7297, t7298, t7301, t7306)
}
