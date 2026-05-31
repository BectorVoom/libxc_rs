//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 85/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk85<F: Float>(t1: F, t44: F, t343: F, t55: F, t78: F, t46: F, t51: F, t345: F, t347: F, t351: F, t353: F, t54: F) -> (F, F, F, F, F, F) {
    let t360 = t44 * t1;
    let t362 = t343 * t78 * t55;
    let t364 = F::cast_from(0.18311555036753159941e-3_f64) * t360 * t362;
    let t365 = t44 * t46;
    let t366 = t51 * t51;
    let t367 = F::cast_from(1.0_f64) / t366;
    let t372 = -F::cast_from(0.86308333333333333334e0_f64) * t345 - F::cast_from(0.301925e0_f64) * t347 - F::cast_from(0.5501625e-1_f64) * t351 - F::cast_from(0.82785e-1_f64) * t353;
    let t374 = F::cast_from(1.0_f64) / t54;
    (t362, t364, t365, t367, t372, t374)
}
