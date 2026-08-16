//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1169/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1169(t33596: f64, t33598: f64, t33604: f64, t22669: f64, t22674: f64, t22676: f64, t22679: f64, t18899: f64, t18961: f64, t18968: f64, t18970: f64, t18973: f64, t18977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48510 = 0.70178680769462448852e1_f64 * t33596;
    let t48511 = 48.0_f64 * t33598;
    let t48512 = 0.65061485296689145287e-1_f64 * t33604;
    let t48513 = 0.13012297059337829057e0_f64 * t22669;
    let t48514 = 0.4155781415850207192e3_f64 * t22674;
    let t48515 = 0.2077890707925103596e3_f64 * t22676;
    let t48516 = 480.0_f64 * t22679;
    let t48517 = -t18961 - t18968 + t18970 + t18973 - t18977 + t48510 - t48511 + t48512 - t48513 + t48514 - t48515 - t48516 - t18899;
    (t48510, t48511, t48512, t48513, t48514, t48515, t48516, t48517)
}
