//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1226/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1226<F: Float>(t1471: F, t1487: F, t1494: F, t5820: F, t5827: F, t5830: F, t5855: F, t5869: F, t71: F, t85: F) -> F {
    let t5872 = -t5820 * t85 / F::cast_from(12.0_f64) - t5827 * t85 / F::cast_from(12.0_f64) - t5830 * t85 / F::cast_from(6.0_f64) - t1471 * t1494 / F::cast_from(6.0_f64) + t5855 * t85 / F::cast_from(24.0_f64) + t1487 * t1494 / F::cast_from(12.0_f64) + t71 * t5869 / F::cast_from(24.0_f64);
    t5872
}
