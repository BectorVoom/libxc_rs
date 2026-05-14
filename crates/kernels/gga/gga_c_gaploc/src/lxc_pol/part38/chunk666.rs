//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 666/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk666<F: Float>(t32743: F, t787: F, t1980: F, t8792: F, t1: F, t106: F, t5745: F, t191: F, t5750: F, t2925: F, t5241: F, t10627: F, t22623: F, t24885: F, t2610: F, t7291: F) -> (F, F, F, F, F, F, F, F) {
    let t32744 = t787 * t32743;
    let t32757 = t1980 * t8792;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32840 = t5241 * t2925;
    let t32847 = t22623 * t10627;
    let t32969 = t787 * t24885;
    let t33087 = t2610 * t7291;
    (t32744, t32757, t32809, t32810, t32840, t32847, t32969, t33087)
}
