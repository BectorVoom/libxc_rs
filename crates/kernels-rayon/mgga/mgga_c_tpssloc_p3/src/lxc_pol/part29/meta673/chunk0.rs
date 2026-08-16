//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2260/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260(t22574: f64, t56120: f64, t8643: f64, t1845: f64, t3719: f64, t1874: f64, t55962: f64, t19456: f64, t6525: f64, t22480: f64, t4028: f64, t26502: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91602 = 3.0_f64 * t22574 * t8643 * t56120;
    let t91603 = t1845 * t3719;
    let t91606 = 3.0_f64 * t22574 * t8643 * t91603;
    let t91608 = 2.0_f64 * t55962 * t1874;
    let t91610 = 4.0_f64 * t19456 * t6525;
    let t91612 = 2.0_f64 * t4028 * t22480;
    let t91620 = t532 * t26502;
    (t91602, t91606, t91608, t91610, t91612, t91620)
}
