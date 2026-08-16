//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 382/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk382(t661: f64, t662: f64, t2309: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t2310 = t661 * t661;
    let t2311 = t2310 * t662;
    let t2313 = 2.0_f64 * t2309 * t2311;
    let t2314 = 1.0_f64 / t128;
    (t2310, t2311, t2313, t2314)
}
