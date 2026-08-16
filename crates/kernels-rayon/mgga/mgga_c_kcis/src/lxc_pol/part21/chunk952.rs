//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 952/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk952(t13495: f64, t4947: f64, t1662: f64, t2911: f64, t9924: f64, t13480: f64, t4939: f64, t2635: f64, t4961: f64, t2894: f64, t1704: f64, t2844: f64) -> (f64, f64, f64, f64, f64) {
    let t14463 = t4947 * t13495;
    let t14466 = t1662 * t2911;
    let t14467 = t9924 * t14466;
    let t14470 = t4939 * t13480;
    let t14473 = t4961 * t2635;
    let t14474 = t2894 * t14473;
    let t14477 = t1704 * t2844;
    (t14463, t14467, t14470, t14474, t14477)
}
