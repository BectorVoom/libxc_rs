//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1076/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1076(t874: f64, t898: f64, t343: f64, t938: f64, t13796: f64, t3989: f64, t2272: f64, t3975: f64, t3972: f64, t328: f64, t922: f64, t356: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13797 = t898 * t874;
    let t13798 = t343 * t938;
    let t13799 = t13797 * t13798;
    let t13800 = t13796 * t13799;
    let t13801 = t3989 * t13800;
    let t13803 = t3975 * t2272;
    let t13804 = t3972 * t13803;
    let t13806 = t328 * t922;
    let t13807 = t356 * t13806;
    (t13798, t13800, t13801, t13803, t13804, t13806, t13807)
}
