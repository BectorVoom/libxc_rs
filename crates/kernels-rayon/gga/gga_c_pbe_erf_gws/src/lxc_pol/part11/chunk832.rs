//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 832/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk832(t1105: f64, t3886: f64, t2376: f64, t2409: f64, t1109: f64, t2501: f64, t829: f64, t830: f64) -> (f64, f64, f64) {
    let t13205 = t1105 * t3886;
    let t13207 = t2409 * t2376 * t13205;
    let t13212 = t829 * t830 * t2501 * t1109;
    (t13205, t13207, t13212)
}
