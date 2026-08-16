//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 715/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk715(t1022: f64, t209: f64, t184: f64, t3440: f64, t401: f64, t3434: f64, t3437: f64, t3342: f64, t4957: f64, t4951: f64, t3422: f64, t395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10742 = t1022 * t209;
    let t10743 = t10742 * t184;
    let t10756 = t401 * t3440;
    let t10758 = t401 * t3434;
    let t10760 = t401 * t3437;
    let t10777 = t4957 * t3342;
    let t10783 = t4951 * t3342;
    let t10823 = t395 * t3422;
    (t10742, t10743, t10756, t10758, t10760, t10777, t10783, t10823)
}
