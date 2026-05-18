//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 121/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk121<F: Float>(t1: F, t44: F, t350: F, t55: F, t78: F, t46: F, t51: F, t352: F, t354: F, t358: F, t360: F, t54: F) -> (F, F, F, F, F, F, F, F) {
    let t367 = t44 * t1;
    let t369 = t350 * t78 * t55;
    let t371 = F::new(0.18311555036753159941e-3) * t367 * t369;
    let t372 = t44 * t46;
    let t373 = t51 * t51;
    let t374 = F::new(1.0) / t373;
    let t379 = -F::new(0.86308333333333333334e0) * t352 - F::new(0.301925e0) * t354 - F::new(0.5501625e-1) * t358 - F::new(0.82785e-1) * t360;
    let t381 = F::new(1.0) / t54;
    (t367, t369, t371, t372, t373, t374, t379, t381)
}
