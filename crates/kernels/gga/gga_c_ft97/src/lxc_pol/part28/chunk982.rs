//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 982/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk982<F: Float>(t138873: F, t527: F, t2058: F, t5551: F, t133: F, t1995: F, t542: F, t138866: F, t5555: F, t8908: F, t128: F, t32796: F) -> (F, F, F, F, F, F, F, F, F) {
    let t139082 = t527 * t138873;
    let t139086 = t2058 * t5551;
    let t139087 = t133 * t139086;
    let t139098 = t1995 * t138873;
    let t139101 = t542 * t139086;
    let t139109 = t527 * t138866;
    let t139115 = t8908 * t5555;
    let t139116 = t133 * t139115;
    let t139121 = t1995 * t138866;
    let t139124 = t542 * t139115;
    let t139131 = t128 * t5551;
    let t139132 = t139131 * t32796;
    (t139082, t139087, t139098, t139101, t139109, t139116, t139121, t139124, t139132)
}
