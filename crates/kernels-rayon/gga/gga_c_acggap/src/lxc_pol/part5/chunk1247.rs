//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1247/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1247(t6220: f64, t997: f64, t6228: f64, t6200: f64, t1140: f64, t5913: f64, t1017: f64, t1089: f64, t1095: f64, t12576: f64, t1323: f64, t1327: f64, t175: f64, t17530: f64, t1795: f64, t18129: f64, t20764: f64, t21673: f64, t335: f64, t367: f64, t418: f64, t4352: f64, t4495: f64, t4593: f64, t5183: f64, t6374: f64, t839: f64, t922: f64, t960: f64) -> f64 {
    let t22890 = t997 * t6220;
    let t22892 = t997 * t6228;
    let t22894 = t997 * t6200;
    let t22906 = t1140 * t5913;
    let t22928 = -0.16006300097412701803e-1_f64 * t22890 + 0.80031500487063509015e-2_f64 * t22892 - 0.12004725073059526352e-1_f64 * t22894 + 0.17149607247227894789e-2_f64 * t418 * t1089 * t1095 * t1795 * t839 - 0.25724410870841842183e-1_f64 * t418 * t4352 * t175 * t21673 - 0.12004725073059526352e-1_f64 * t17530 - 7.0_f64 / 36.0_f64 * t22906 + t335 * t18129 * t1323 / 12.0_f64 + t335 * t4593 * t5183 / 12.0_f64 + t335 * t4593 * t4495 / 24.0_f64 + t367 * t18129 * t1327 / 12.0_f64 + 5.0_f64 / 4.0_f64 * t12576 * t960 * t6374 * t922 - t367 * t960 * t20764 * t1017 / 16.0_f64;
    t22928
}
