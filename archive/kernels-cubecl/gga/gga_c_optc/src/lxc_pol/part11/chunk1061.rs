//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1061/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1061<F: Float>(t1291: F, t23163: F, t1278: F, t23171: F, t1264: F, t2086: F, t3386: F, t6642: F, t6751: F, t9529: F, t6739: F, t6825: F) -> (F, F, F, F, F, F, F) {
    let t29128 = t23163 * t1291;
    let t29210 = t23171 * t1278;
    let t29284 = t1264 * t2086;
    let t29330 = t3386 * t6642;
    let t29335 = t9529 * t6751;
    let t29341 = t9529 * t6739;
    let t29346 = t3386 * t6825;
    (t29128, t29210, t29284, t29330, t29335, t29341, t29346)
}
