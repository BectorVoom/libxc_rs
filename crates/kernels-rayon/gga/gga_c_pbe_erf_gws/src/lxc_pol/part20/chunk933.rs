//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 933/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk933(t10456: f64, t5211: f64, t10402: f64, t7759: f64, t7115: f64, t2666: f64, t7106: f64, t1022: f64, t7483: f64, t2673: f64, t10417: f64, t10421: f64, t10423: f64, t10428: f64, t10432: f64, t10436: f64, t10441: f64, t10446: f64, t10450: f64, t10454: f64, t5205: f64, t7190: f64, t7193: f64) -> (f64, f64, f64, f64, f64) {
    let t10458 = 16.0_f64 / 45.0_f64 * t5211 * t10456;
    let t10459 = t7759 * t10402;
    let t10461 = 8.0_f64 / 27.0_f64 * t7115 * t10459;
    let t10462 = t7106 * t2666;
    let t10464 = 16.0_f64 / 45.0_f64 * t5211 * t10462;
    let t10465 = t7483 * t1022;
    let t10466 = t10465 * t2673;
    let t10468 = 32.0_f64 / 45.0_f64 * t5211 * t10466;
    let t10469 = t10417 + t10421 + 2.0_f64 / 135.0_f64 * t5205 + t10423 - t7190 + t7193 + t10428 + t10432 - t10436 - t10441 + t10446 + t10450 + t10454 - t10458 - t10461 - t10464 - t10468;
    (t10458, t10461, t10464, t10468, t10469)
}
