//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 590/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk590<F: Float>(t1528: F, t1912: F, t259: F, t4147: F, t4268: F, t6549: F, t6565: F, t6627: F, t7481: F, t7486: F, t7490: F, t7492: F, t7511: F, t7517: F, t7538: F, t855: F) -> F {
    let t7540 = -t6549 - F::cast_from(0.16449340668482264365e-1_f64) * t7481 - t6565 + F::cast_from(0.82246703342411321825e-2_f64) * t7486 - F::cast_from(0.82246703342411321825e-2_f64) * t7490 + t7492 * t259 + t7511 * t259 - t6627 * t1528 - t4147 * t1912 - t4268 * t1912 + F::cast_from(2.0_f64) * t855 * t7517 - t855 * t7538;
    t7540
}
