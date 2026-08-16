//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1185/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1185<F: Float>(t1772: F, t336: F, t1017: F, t1150: F, t1180: F, t1181: F, t13181: F, t13184: F, t13192: F, t15995: F, t20972: F, t21528: F, t21530: F, t21532: F, t3282: F, t335: F, t367: F, t4483: F, t4593: F, t4643: F, t5606: F, t6289: F, t6293: F, t960: F, t961: F) -> F {
    let t21537 = t336 * t1772;
    let t21550 = F::cast_from(0.34299214494455789578e-2_f64) * t1180 * t1181 * t15995 * t5606 + F::cast_from(0.34299214494455789578e-2_f64) * t1180 * t1181 * t4643 * t20972 + F::cast_from(0.42874018118069736972e-3_f64) * t13181 + F::cast_from(0.34013387707001991332e-1_f64) * t13184 + F::cast_from(0.12862205435420921092e-2_f64) * t13192 - F::cast_from(0.80031500487063509015e-2_f64) * t21528 + F::cast_from(0.80031500487063509015e-2_f64) * t21530 + t1150 * t960 * t21532 * t1017 / F::cast_from(8.0_f64) + t335 * t21537 * t961 / F::cast_from(24.0_f64) + t1150 * t4593 * t4483 / F::cast_from(8.0_f64) + t335 * t3282 * t6289 / F::cast_from(24.0_f64) + t367 * t3282 * t6293 / F::cast_from(24.0_f64);
    t21550
}
