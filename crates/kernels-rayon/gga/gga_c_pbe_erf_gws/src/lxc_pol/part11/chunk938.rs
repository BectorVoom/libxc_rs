//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 938/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk938(t2276: f64, t2299: f64, t6201: f64, t20269: f64, t932: f64, t2200: f64, t863: f64, t864: f64, t322: f64, t6382: f64, t274: f64, t6094: f64) -> (f64, f64, f64, f64, f64) {
    let t20944 = t2276 * t6201 * t2299;
    let t20948 = t2276 * t20269 * t932;
    let t20962 = t863 * t864 * t2200;
    let t21010 = t322 * t6382;
    let t21091 = t322 / t6094 / t274;
    (t20944, t20948, t20962, t21010, t21091)
}
