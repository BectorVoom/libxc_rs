//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 678/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk678<F: Float>(t32435: F, t7290: F, t2958: F, t7291: F, t688: F, t123: F, t24884: F, t10627: F, t1858: F, t787: F, t1980: F, t8792: F, t1: F, t106: F, t5745: F, t191: F, t5750: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32436 = t7290 * t32435;
    let t32607 = t2958 * t7291;
    let t32658 = t32435 * t688;
    let t32692 = t24884 * t123;
    let t32743 = t1858 * t10627;
    let t32744 = t787 * t32743;
    let t32757 = t1980 * t8792;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    (t32436, t32607, t32658, t32692, t32743, t32744, t32757, t32809, t32810)
}
