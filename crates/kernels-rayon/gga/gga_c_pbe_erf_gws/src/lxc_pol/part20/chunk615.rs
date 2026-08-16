//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 615/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk615(t3140: f64, t3219: f64, t3235: f64, t254: f64, t3240: f64, t906: f64, t2079: f64, t6: f64) -> (f64, f64, f64) {
    let t3249 = t3235 * t3219 * t3140;
    let t3252 = t254 * t3240;
    let t3253 = t3252 * t906;
    let t3257 = t254 * t6 * t2079;
    (t3249, t3253, t3257)
}
