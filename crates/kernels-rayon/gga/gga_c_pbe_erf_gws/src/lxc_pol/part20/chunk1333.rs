//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1333/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1333(t11535: f64, t14031: f64, t3128: f64, t54359: f64, t14570: f64, t9111: f64, t11824: f64, t14015: f64, t11466: f64, t14011: f64, t11578: f64, t14498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57042 = t14031 * t11535;
    let t57044 = t3128 * t54359;
    let t57046 = t9111 * t14570;
    let t57048 = t14015 * t11824;
    let t57050 = t14011 * t11466;
    let t57052 = t14498 * t11578;
    (t57042, t57044, t57046, t57048, t57050, t57052)
}
