//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 800/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk800<F: Float>(t2348: F, t6627: F, t343: F, t816: F, t874: F, t2084: F, t3257: F, t2251: F, t916: F, t2250: F) -> (F, F, F, F) {
    let t6628 = t6627 * t2348;
    let t6631 = t816 * t874 * t343;
    let t6632 = t2084 * t6631;
    let t6633 = t3257 * t6632;
    let t6636 = t2251 * t916;
    let t6637 = t2250 * t6636;
    (t6628, t6633, t6636, t6637)
}
