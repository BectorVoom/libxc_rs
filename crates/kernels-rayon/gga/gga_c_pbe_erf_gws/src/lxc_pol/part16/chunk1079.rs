//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1079/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1079(t1178: f64, t2402: f64, t371: f64, t13830: f64, t2409: f64, t6782: f64, t3965: f64, t1176: f64, t2344: f64, t367: f64) -> (f64, f64, f64, f64, f64) {
    let t13832 = t371 * t1178 * t2402;
    let t13833 = t13830 * t13832;
    let t13855 = t2409 * t6782;
    let t13856 = t3965 * t13855;
    let t13859 = t1176 * t367 * t2344;
    (t13832, t13833, t13855, t13856, t13859)
}
