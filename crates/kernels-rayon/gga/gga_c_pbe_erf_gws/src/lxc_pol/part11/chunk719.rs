//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 719/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk719(t3493: f64, t636: f64, t3443: f64, t597: f64, t3534: f64, t5018: f64, t1820: f64, t3522: f64, t5480: f64, t639: f64, t1630: f64, t3518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10889 = t3493 * t636;
    let t10908 = t597 * t3443;
    let t10913 = t5018 * t3534;
    let t10914 = t1820 * t10913;
    let t10924 = t5480 * t3522;
    let t10925 = t639 * t10924;
    let t10927 = t1630 * t3518;
    (t10889, t10908, t10913, t10914, t10924, t10925, t10927)
}
