//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 763/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk763(t211: f64, t5322: f64, t1879: f64, t1882: f64, t1748: f64, t202: f64, t184: f64, t1871: f64, t582: f64, t561: f64, t1680: f64, t583: f64) -> (f64, f64, f64, f64, f64) {
    let t5323 = t211 * t5322;
    let t5338 = t1879 * t1882;
    let t5342 = t202 * t1748;
    let t5343 = t5342 * t184;
    let t5346 = t582 * t1871;
    let t5347 = t561 * t5346;
    let t5349 = t1680 * t583;
    (t5323, t5338, t5343, t5347, t5349)
}
