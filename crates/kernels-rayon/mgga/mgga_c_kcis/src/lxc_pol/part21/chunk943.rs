//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 943/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk943(t13791: f64, t14326: f64, t10416: f64, t1045: f64, t14302: f64, t3255: f64, t4576: f64, t4582: f64, t4568: f64, t13462: f64, t4565: f64, t10386: f64, t347: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14327 = t14326 * t13791;
    let t14331 = t10416 * t14302 * t1045;
    let t14339 = 0.8760572888888888889e-3_f64 * t3255 * t4576;
    let t14341 = 0.17521145777777777778e-2_f64 * t3255 * t4582;
    let t14343 = 0.14600954814814814815e-2_f64 * t3255 * t4568;
    let t14344 = t4565 * t13462;
    let t14347 = t10386 * t347;
    (t14327, t14331, t14339, t14341, t14343, t14344, t14347)
}
