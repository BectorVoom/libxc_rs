//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 845/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk845(t22410: f64, t2843: f64, t840: f64, t21362: f64, t319: f64, t835: f64, t14946: f64, t21947: f64, t21951: f64, t21955: f64, t21960: f64, t21964: f64, t21967: f64, t21971: f64, t21975: f64, t21984: f64, t21987: f64, t21991: f64, t21994: f64) -> (f64, f64, f64) {
    let t22412 = t840 * t2843 * t22410;
    let t22416 = t835 * t319 * t21362;
    let t22432 = 2.0_f64 / 3.0_f64 * t21994 + t21971 / 3.0_f64 + t21975 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t21960 - 2.0_f64 / 9.0_f64 * t21967 - 2.0_f64 / 3.0_f64 * t21947 - 2.0_f64 / 3.0_f64 * t21951 - 10.0_f64 / 81.0_f64 * t21955 + 4.0_f64 / 9.0_f64 * t21964 + 2.0_f64 * t21984 - t21987 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t21991 - t14946;
    (t22412, t22416, t22432)
}
