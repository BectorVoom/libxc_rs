//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 889/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk889<F: Float>(t1912: F, t25168: F, t259: F, t26713: F, t30655: F, t30662: F, t31350: F, t32865: F, t32869: F, t33405: F, t33410: F, t33412: F, t33414: F, t6627: F, t7842: F) -> F {
    let t33416 = -t6627 * t7842 - t26713 * t1912 - F::cast_from(6.0_f64) * t25168 * t33405 - F::cast_from(0.82246703342411321825e-2_f64) * t33410 - t30655 + t32865 - t32869 + t30662 - t31350 + t33412 * t259 + t33414 * t259;
    t33416
}
