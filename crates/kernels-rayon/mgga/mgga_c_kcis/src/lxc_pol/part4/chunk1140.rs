//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1140/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1140(t14453: f64, t4952: f64, t991: f64, t3040: f64, t4966: f64, t417: f64, t13495: f64, t4947: f64, t1662: f64, t2911: f64, t9924: f64, t13480: f64, t4939: f64) -> (f64, f64, f64, f64, f64) {
    let t14454 = t14453 * t4952;
    let t14455 = t991 * t14454;
    let t14459 = t4966 * t3040;
    let t14460 = t417 * t14459;
    let t14463 = t4947 * t13495;
    let t14466 = t1662 * t2911;
    let t14467 = t9924 * t14466;
    let t14470 = t4939 * t13480;
    (t14455, t14460, t14463, t14467, t14470)
}
