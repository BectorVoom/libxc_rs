//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 810/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk810<F: Float>(t13076: F, t13091: F, t44: F, t251: F, t691: F, t102: F, t4880: F, t23: F, t821: F, t6: F, t107: F, t2621: F, t9: F) -> (F, F, F, F, F) {
    let t13093 = (t13076 + t13091) * t44;
    let t13396 = t691 * t251;
    let t13577 = t102 * t4880;
    let t13581 = F::cast_from(1.0_f64) / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    let t13587 = F::cast_from(1.0_f64) / t9 / t2621;
    (t13093, t13396, t13577, t13583, t13587)
}
