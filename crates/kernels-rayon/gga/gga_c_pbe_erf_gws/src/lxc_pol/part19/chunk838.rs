//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 838/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk838(t1383: f64, t992: f64, t168: f64, t2831: f64, t703: f64, t1072: f64, t1472: f64, t142: f64, t2873: f64, t2893: f64, t501: f64, t156: f64, t4: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8058 = t992 * t1383;
    let t8064 = 0.39794582218349216586e-1_f64 * t168 * t703 * t2831;
    let t8066 = t168 * t1472 * t1072;
    let t8108 = t142 * t2873;
    let t8122 = t501 * t2893;
    let t8124 = t4 * t156 * t481;
    (t8058, t8064, t8066, t8108, t8122, t8124)
}
