//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 685/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk685<F: Float>(t102: F, t6599: F, t108: F, t176: F, t203: F, t1864: F, t587: F, t6407: F, t601: F, t6424: F, t6427: F, t580: F, t6419: F) -> (F, F, F, F, F, F, F) {
    let t6600 = t6599 * t102;
    let t6602 = t176 * t6600 * t108;
    let t6604 = t6602 * t203 / F::new(2.0);
    let t6617 = t1864 * t6407 * t587;
    let t6619 = F::new(0.35089340384731224426e1) * t601 * t6617;
    let t6636 = t6424 * t6407 * t6427;
    let t6638 = F::new(0.1025389702100779493e4) * t601 * t6636;
    let t6642 = t580 * t6419 * t587;
    (t6602, t6604, t6617, t6619, t6636, t6638, t6642)
}
