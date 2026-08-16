//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 969/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk969(t11235: f64, t4018: f64, t11234: f64, t619: f64, t640: f64, t2941: f64, t128: f64, t200: f64, t1954: f64, t2922: f64, t2903: f64, t3635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11236 = t11235 * t4018;
    let t11237 = t11234 * t11236;
    let t11239 = t640 * t619;
    let t11240 = t2941 * t11239;
    let t11242 = t128 * t200;
    let t11243 = t11242 * t1954;
    let t11244 = t2922 * t11243;
    let t11246 = t2903 * t3635;
    (t11236, t11237, t11239, t11240, t11242, t11243, t11244, t11246)
}
