//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 674/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk674(t1045: f64, t1672: f64, t211: f64, t219: f64, t5400: f64, t5480: f64, t1663: f64, t995: f64, t1023: f64, t616: f64, t996: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7844 = t1672 * t1045;
    let t7845 = t211 * t7844;
    let t7853 = t5400 * t219;
    let t7877 = t5480 * t219;
    let t7899 = t995 * t1663;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    (t7844, t7845, t7853, t7877, t7899, t7945, t7946, t7956, t7957)
}
