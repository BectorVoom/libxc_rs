//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 248/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk248(t286: f64, t751: f64, t159: f64, t285: f64, t535: f64, t147: f64, t545: f64, t281: f64, t532: f64) -> (f64, f64, f64, f64, f64) {
    let t753 = 0.19957056683757681823e-1_f64 * t751 * t286;
    let t755 = t535 * t159 * t285;
    let t759 = t147 * t545 * t285;
    let t761 = 0.11974234010254609094e-1_f64 * t281 * t759;
    let t762 = t532 * t147;
    (t753, t755, t759, t761, t762)
}
