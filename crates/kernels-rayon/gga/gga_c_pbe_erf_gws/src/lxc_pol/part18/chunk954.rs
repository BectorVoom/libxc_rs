//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 954/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk954(t1024: f64, t7951: f64, t10711: f64, t10714: f64, t10717: f64, t10721: f64, t10726: f64, t10728: f64, t10730: f64, t10732: f64, t10734: f64, t10736: f64, t10738: f64, t10739: f64, t10741: f64, t10745: f64, t10749: f64, t7578: f64) -> (f64, f64) {
    let t10751 = 8.0_f64 / 15.0_f64 * t7951 * t1024;
    let t10752 = -t10711 - t10714 + t10717 + t10721 + t10726 + t10728 - t10730 - t10732 + t10734 + t10736 + t10738 + t7578 - t10739 - t10741 - t10745 + t10749 + t10751;
    (t10751, t10752)
}
