//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3713/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713<F: Float>(t1261: F, t20981: F, t3172: F, t13033: F, t21188: F, t20985: F, t20820: F, t3704: F, t17720: F, t5381: F, t17214: F, t17505: F, t17536: F, t17552: F, t17786: F, t20941: F, t21275: F, t21306: F, t3591: F, t44561: F, t5299: F, t5391: F, t57118: F, t58927: F) -> F {
    let t70369 = t1261 * t3172 * t20981;
    let t70373 = t13033 * t21188;
    let t70376 = t1261 * t3172 * t20985;
    let t70378 = t20820 * t3704;
    let t70382 = t5381 * t17720;
    let t70390 = F::cast_from(0.57165357490759649296e-3_f64) * t44561 * t20941 + F::cast_from(0.1270341277572436651e-3_f64) * t57118 - F::cast_from(0.42874018118069736972e-3_f64) * t21306 * t17786 - F::cast_from(0.76220476654346199061e-3_f64) * t70369 + F::cast_from(0.21437009059034868486e-3_f64) * t20820 * t3591 + F::cast_from(0.57165357490759649296e-3_f64) * t70373 - F::cast_from(0.11433071498151929859e-2_f64) * t70376 + F::cast_from(0.28582678745379824648e-3_f64) * t70378 - F::cast_from(0.15244095330869239812e-1_f64) * t5391 * t17552 + F::cast_from(0.6351706387862183255e-3_f64) * t70382 - F::cast_from(0.57165357490759649296e-3_f64) * t21275 * t17214 - F::cast_from(0.30488190661738479624e-2_f64) * t58927 * t5299 - F::cast_from(0.30488190661738479624e-2_f64) * t17505 * t17536;
    t70390
}
