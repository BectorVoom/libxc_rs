//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1103/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1103(t343: f64, t938: f64, t328: f64, t922: f64, t356: f64, t3971: f64) -> (f64, f64, f64, f64) {
    let t13798 = t343 * t938;
    let t13806 = t328 * t922;
    let t13807 = t356 * t13806;
    let t13808 = t13807 * t3971;
    (t13798, t13806, t13807, t13808)
}
