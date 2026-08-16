//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3271/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3271(t14686: f64, t18525: f64, t50570: f64, t61956: f64, t14923: f64, t18428: f64, t10760: f64, t40627: f64, t61837: f64, t18527: f64, t50295: f64, t18444: f64, t2745: f64, t2754: f64, t40801: f64, t40804: f64, t40810: f64, t4364: f64, t51000: f64, t51006: f64, t51026: f64, t51028: f64) -> f64 {
    let t62105 = t50570 * t14686 * t61956 * t18525;
    let t62108 = t14923 * t18428;
    let t62111 = t10760 * t40627 * t61837;
    let t62114 = t50295 * t18527;
    let t62123 = -0.12004725073059526352e-1_f64 * t51000 - 0.15246000842785598468e-3_f64 * t62105 - 0.80031500487063509015e-2_f64 * t51006 + 0.16006300097412701803e-1_f64 * t62108 + 0.36143185997963725434e-4_f64 * t62111 + 0.90357964994909313582e-5_f64 * t40801 + 0.12004725073059526352e-1_f64 * t62114 - 0.50820002809285328225e-4_f64 * t40804 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t18444 * t2754 + t40810 - 7.0_f64 / 12.0_f64 * t51026 - 7.0_f64 / 24.0_f64 * t51028;
    t62123
}
