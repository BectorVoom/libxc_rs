//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1135/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1135(t119792: f64, t126078: f64, t828: f64, t855: f64, t119818: f64, t1561: f64, t31846: f64, t4426: f64, t119777: f64, t4430: f64, t119788: f64, t1558: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126081 = t119792 * t855 * t828 * t126078;
    let t126083 = t119818 * t1561;
    let t126085 = t31846 * t4426;
    let t126087 = t119777 * t4430;
    let t126089 = t119788 * t4430;
    let t126092 = t867 * t1558;
    (t126081, t126083, t126085, t126087, t126089, t126092)
}
