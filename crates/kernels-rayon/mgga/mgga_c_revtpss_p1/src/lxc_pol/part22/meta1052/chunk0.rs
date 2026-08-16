//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3713/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713(t1261: f64, t20981: f64, t3172: f64, t13033: f64, t21188: f64, t20985: f64, t20820: f64, t3704: f64, t17720: f64, t5381: f64, t17214: f64, t17505: f64, t17536: f64, t17552: f64, t17786: f64, t20941: f64, t21275: f64, t21306: f64, t3591: f64, t44561: f64, t5299: f64, t5391: f64, t57118: f64, t58927: f64) -> f64 {
    let t70369 = t1261 * t3172 * t20981;
    let t70373 = t13033 * t21188;
    let t70376 = t1261 * t3172 * t20985;
    let t70378 = t20820 * t3704;
    let t70382 = t5381 * t17720;
    let t70390 = 0.57165357490759649296e-3_f64 * t44561 * t20941 + 0.1270341277572436651e-3_f64 * t57118 - 0.42874018118069736972e-3_f64 * t21306 * t17786 - 0.76220476654346199061e-3_f64 * t70369 + 0.21437009059034868486e-3_f64 * t20820 * t3591 + 0.57165357490759649296e-3_f64 * t70373 - 0.11433071498151929859e-2_f64 * t70376 + 0.28582678745379824648e-3_f64 * t70378 - 0.15244095330869239812e-1_f64 * t5391 * t17552 + 0.6351706387862183255e-3_f64 * t70382 - 0.57165357490759649296e-3_f64 * t21275 * t17214 - 0.30488190661738479624e-2_f64 * t58927 * t5299 - 0.30488190661738479624e-2_f64 * t17505 * t17536;
    t70390
}
