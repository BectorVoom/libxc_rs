//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 951/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk951(t14443: f64, t4581: f64, t991: f64, t291: f64, t9959: f64, t4567: f64, t2469: f64, t992: f64, t4952: f64, t3040: f64, t4966: f64, t417: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14444 = t14443 * t4581;
    let t14446 = t991 * t14444 / 216.0_f64;
    let t14447 = t9959 * t291;
    let t14448 = t14447 * t4567;
    let t14450 = t991 * t14448 / 324.0_f64;
    let t14453 = t2469 * t992;
    let t14454 = t14453 * t4952;
    let t14455 = t991 * t14454;
    let t14459 = t4966 * t3040;
    let t14460 = t417 * t14459;
    (t14446, t14447, t14450, t14453, t14455, t14460)
}
