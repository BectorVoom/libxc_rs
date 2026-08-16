//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1278/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1278(t1281: f64, t29186: f64, t100026: f64, t100029: f64, t100031: f64, t100033: f64, t1291: f64, t15692: f64, t2205: f64, t28073: f64, t28253: f64, t28295: f64, t29188: f64, t29214: f64, t34650: f64, t35615: f64, t3664: f64, t3669: f64, t47711: f64, t5360: f64, t6879: f64, t70451: f64, t7823: f64, t8108: f64, t99859: f64, t99861: f64, t99864: f64) -> f64 {
    let t100920 = t29186 * t1281;
    let t100927 = 24.0_f64 * t1291 * t29188 * t35615 + 2.0_f64 * t1291 * t29214 * t3669 + 2.0_f64 * t3669 * t6879 * t7823 - t100920 * t1291 + 4.0_f64 * t15692 * t28073 + 4.0_f64 * t15692 * t28253 - t2205 * t70451 - 2.0_f64 * t28295 * t5360 - 6.0_f64 * t29188 * t34650 - t29214 * t3664 + 4.0_f64 * t47711 * t8108 + t100026 + t100029 + t100031 - t100033 - t99859 - t99861 + t99864;
    t100927
}
