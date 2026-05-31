//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 845/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk845<F: Float>(t1319: F, t1410: F, t1897: F, t3821: F, t3824: F, t456: F, t5481: F, t5503: F, t5510: F, t5523: F) -> F {
    let t5526 = -t3821 * t5503 / F::cast_from(8.0_f64) + t3824 * t1897 / F::cast_from(4.0_f64) + t1410 * t5481 / F::cast_from(4.0_f64) + t5510 * t1319 / F::cast_from(4.0_f64) + t456 * t5523 / F::cast_from(2.0_f64);
    t5526
}
