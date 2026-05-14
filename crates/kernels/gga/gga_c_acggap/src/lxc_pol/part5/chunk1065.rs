//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1065/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1065<F: Float>(t1772: F, t336: F, t1017: F, t1150: F, t1180: F, t1181: F, t13181: F, t13184: F, t13192: F, t15995: F, t20972: F, t21528: F, t21530: F, t21532: F, t3282: F, t335: F, t367: F, t4483: F, t4593: F, t4643: F, t5606: F, t6289: F, t6293: F, t960: F, t961: F) -> (F,) {
    let t21537 = t336 * t1772;
    let t21550 = 0.34299214494455789578e-2 * t1180 * t1181 * t15995 * t5606 + 0.34299214494455789578e-2 * t1180 * t1181 * t4643 * t20972 + 0.42874018118069736972e-3 * t13181 + 0.34013387707001991332e-1 * t13184 + 0.12862205435420921092e-2 * t13192 - 0.80031500487063509015e-2 * t21528 + 0.80031500487063509015e-2 * t21530 + t1150 * t960 * t21532 * t1017 / 8.0 + t335 * t21537 * t961 / 24.0 + t1150 * t4593 * t4483 / 8.0 + t335 * t3282 * t6289 / 24.0 + t367 * t3282 * t6293 / 24.0;
    (t21550,)
}
