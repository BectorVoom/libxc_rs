//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 513/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk513(t2264: f64, t366: f64, t899: f64, t2158: f64, t904: f64, t916: f64, t745: f64, t823: f64) -> (f64, f64, f64) {
    let t2266 = t899 * t2264 * t366;
    let t2268 = t916 * t904 * t2158;
    let t2271 = t823 * t745;
    (t2266, t2268, t2271)
}
