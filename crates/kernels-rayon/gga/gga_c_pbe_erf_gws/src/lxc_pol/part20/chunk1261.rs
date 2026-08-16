//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1261/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1261(t54491: f64, t898: f64, t911: f64, t3973: f64, t13953: f64, t14787: f64, t14781: f64, t14001: f64, t3062: f64, t14772: f64, t1161: f64, t353: f64, t51084: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54492 = 7.0_f64 / 2304.0_f64 * t54491;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54504 = t13953 * t14787;
    let t54505 = 7.0_f64 / 144.0_f64 * t54504;
    let t54531 = t13953 * t14781;
    let t54532 = 7.0_f64 / 144.0_f64 * t54531;
    let t54535 = t14001 * t3062;
    let t54536 = 7.0_f64 / 72.0_f64 * t54535;
    let t54537 = t14001 * t14772;
    let t54538 = 7.0_f64 / 72.0_f64 * t54537;
    let t54545 = t859 * t353 * t51084 * t1161;
    (t54492, t54499, t54505, t54532, t54536, t54538, t54545)
}
