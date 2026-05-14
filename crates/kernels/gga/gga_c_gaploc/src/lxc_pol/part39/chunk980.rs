//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 980/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk980<F: Float>(t1897: F, t39454: F, t954: F, t2508: F, t47130: F, t688: F, t779: F, t12213: F, t2580: F, t7291: F, t12218: F, t7226: F, t13937: F, t2549: F, t12176: F, t2558: F, t943: F) -> (F, F, F, F, F, F) {
    let t47673 = t1897 * t954 * t39454;
    let t47677 = t2508 * t779 * t47130 * t688;
    let t47681 = t2508 * t2580 * t12213 * t7291;
    let t47685 = t2508 * t7226 * t12218 * t7291;
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    (t47673, t47677, t47681, t47685, t47687, t47690)
}
