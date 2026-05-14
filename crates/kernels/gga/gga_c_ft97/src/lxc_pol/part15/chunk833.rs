//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 833/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk833<F: Float>(t1586: F, t20098: F, t20461: F, t487: F, t1882: F, t20413: F, t20215: F, t8392: F, t20431: F, t20403: F, t8417: F, t20421: F, t20424: F, t20248: F, t20284: F, t20188: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t74389 = t1586 * t20098;
    let t74690 = t20461 * t487;
    let t74745 = t1882 * t20413;
    let t74755 = t8392 * t20215;
    let t74757 = t8392 * t20431;
    let t74759 = t8417 * t20403;
    let t74778 = t1882 * t20421;
    let t74780 = t1882 * t20424;
    let t74786 = t1882 * t20248;
    let t74809 = t1882 * t20284;
    let t74861 = t1882 * t20188;
    (t74389, t74690, t74745, t74755, t74757, t74759, t74778, t74780, t74786, t74809, t74861)
}
