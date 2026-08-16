//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 893/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk893(t155: f64, t1660: f64, t1665: f64, t587: f64, t5009: f64, t5283: f64, t1804: f64, t1866: f64, t1885: f64, t5175: f64, t1652: f64, t5304: f64) -> (f64, f64, f64, f64) {
    let t16942 = t155 * t1660;
    let t16944 = t587 * t16942 * t1665;
    let t16945 = 16.0_f64 / 81.0_f64 * t16944;
    let t16947 = t587 * t5283 * t5009;
    let t16948 = 64.0_f64 / 27.0_f64 * t16947;
    let t16953 = 24.0_f64 / 5.0_f64 * t587 * t1885 * t5175 * t1804 * t1866;
    let t16954 = t5304 * t1652;
    (t16945, t16948, t16953, t16954)
}
