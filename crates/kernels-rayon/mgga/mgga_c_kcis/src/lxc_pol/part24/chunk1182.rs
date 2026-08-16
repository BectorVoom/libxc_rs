//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1182/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1182(t27750: f64, t27753: f64, t27756: f64, t1141: f64, t27985: f64, t283: f64, t5164: f64, t5082: f64, t982: f64, t14781: f64, t1796: f64, t26929: f64, t5025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95279 = t27750 / 8.0_f64;
    let t95280 = t27753 / 8.0_f64;
    let t95281 = t27756 / 8.0_f64;
    let t95286 = t27985 * t1141;
    let t95321 = t5164 * t283;
    let t95326 = t5082 * t982;
    let t95351 = t14781 * t283;
    let t95376 = t1796 * t982;
    let t95381 = t5025 * t26929;
    (t95279, t95280, t95281, t95286, t95321, t95326, t95351, t95376, t95381)
}
