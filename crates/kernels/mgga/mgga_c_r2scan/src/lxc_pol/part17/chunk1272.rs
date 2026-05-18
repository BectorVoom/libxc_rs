//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1272/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1272<F: Float>(t39290: F, t44083: F, t44086: F, t44089: F, t44091: F, t44093: F, t44096: F, t44098: F, t44100: F, t44103: F, t44108: F, t44110: F, t44113: F, t44115: F, t44117: F) -> F {
    let t44964 = -t44083 + t44086 + t44089 - t44091 + t44093 - t44096 + t44098 - t44100 + t44103 + F::new(0.12195059916630011325e-2) * t39290 + t44108 - t44110 - t44113 + t44115 - t44117;
    t44964
}
