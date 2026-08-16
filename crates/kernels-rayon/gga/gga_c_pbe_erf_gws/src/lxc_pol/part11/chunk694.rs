//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 694/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk694(t1154: f64, t6455: f64, t254: f64, t6: f64, t6469: f64, t1113: f64, t904: f64) -> (f64, f64, f64) {
    let t9457 = t6455 * t1154;
    let t9482 = t254 * t6 * t6469;
    let t9499 = t904 * t1113;
    (t9457, t9482, t9499)
}
