//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 877/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk877<F: Float>(t11727: F, t11730: F, t11736: F, t1319: F, t1410: F, t16194: F, t16483: F, t16488: F, t16491: F, t16500: F, t16503: F, t16530: F, t1897: F, t3781: F, t3809: F, t3821: F, t3824: F, t456: F, t5481: F, t5503: F, t5510: F) -> F {
    let t16533 = F::new(3.0) / F::new(16.0) * t11727 * t16483 - t11730 * t5503 / F::new(4.0) - t3821 * t16488 / F::new(4.0) - t3821 * t16491 / F::new(8.0) + t11736 * t1897 / F::new(4.0) + t3824 * t5481 / F::new(2.0) + t1410 * t16194 / F::new(4.0) - t16500 * t3781 / F::new(8.0) + t16503 * t1319 / F::new(2.0) + t5510 * t3809 / F::new(4.0) + t456 * t16530 / F::new(2.0);
    t16533
}
