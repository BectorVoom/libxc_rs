//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1339/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1339(t11223: f64, t15095: f64, t15098: f64, t1872: f64, t27100: f64, t27141: f64, t28073: f64, t28253: f64, t28260: f64, t28295: f64, t34650: f64, t3664: f64, t47711: f64, t5394: f64, t7812: f64, t92410: f64, t95503: f64, t95506: f64, t95508: f64, t95510: f64, t95514: f64, t95517: f64, t95520: f64, t96542: f64, t96545: f64) -> f64 {
    let t96708 = 4.0_f64 * t11223 * t28073 + 4.0_f64 * t11223 * t28253 + 2.0_f64 * t15095 * t27141 + 4.0_f64 * t15098 * t27141 - t1872 * t92410 - 2.0_f64 * t27100 * t5394 - 12.0_f64 * t28260 * t34650 - 2.0_f64 * t28295 * t3664 + 4.0_f64 * t47711 * t7812 + t95503 + t95506 - t95508 - t95510 - t95514 - t95517 - t95520 - t96542 - t96545;
    t96708
}
