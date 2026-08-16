//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1242/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1242(t1014: f64, t26828: f64, t26851: f64, t26972: f64, t7768: f64, t1141: f64, t26866: f64, t14443: f64, t26702: f64, t26685: f64, t7703: f64, t330: f64, t9985: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93211 = t1014 * t26828;
    let t93216 = t1014 * t26851;
    let t93222 = t7768 * t26972;
    let t93243 = t26866 * t1141;
    let t93341 = t14443 * t26702;
    let t93342 = t26685 * t93341;
    let t93344 = t7703 * t93341;
    let t93346 = t9985 * t330;
    (t93211, t93216, t93222, t93243, t93342, t93344, t93346)
}
