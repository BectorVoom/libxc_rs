//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1350/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1350<F: Float>(t10617: F, t2188: F, t809: F, t10616: F, t2236: F, t2234: F, t10655: F, t6574: F, t10659: F, t20824: F, t8611: F, t8923: F) -> (F, F, F, F, F) {
    let t29426 = F::cast_from(4.0_f64) * t2188 * t10617 * t809;
    let t29427 = t10616 * t2236;
    let t29430 = F::cast_from(0.32163958997385070134e2_f64) * t2234 * t29427 * t809;
    let t29432 = F::cast_from(0.64327917994770140268e2_f64) * t6574 * t10655;
    let t29434 = F::cast_from(0.1034520258385468006e4_f64) * t20824 * t10659;
    let t29436 = F::cast_from(4.0_f64) * t8923 * t8611;
    (t29426, t29430, t29432, t29434, t29436)
}
