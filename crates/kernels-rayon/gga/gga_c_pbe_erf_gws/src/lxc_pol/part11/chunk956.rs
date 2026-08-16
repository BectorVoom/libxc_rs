//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 956/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk956(t1069: f64, t5519: f64, t3013: f64, t713: f64, t242: f64, t8279: f64, t2: f64, t39: f64, t967: f64, t19383: f64, t2704: f64, t2863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25395 = t1069 * t5519;
    let t25514 = t3013 * t713;
    let t25569 = t8279 * t242;
    let t25593 = t967 * t2 * t39;
    let t25594 = t19383 * t25593;
    let t25608 = t2863 * t2704;
    (t25395, t25514, t25569, t25593, t25594, t25608)
}
