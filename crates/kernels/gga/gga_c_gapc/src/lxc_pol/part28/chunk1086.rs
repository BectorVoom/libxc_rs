//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1086/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1086<F: Float>(t19159: F, t442: F, t8139: F, t2642: F, t2763: F, t2766: F, t2315: F, t7389: F, t672: F, t818: F, t1087: F, t2299: F) -> (F, F, F, F, F) {
    let t19161 = t8139 * t19159 * t442;
    let t19179 = M_PI * t2642 * t2763 * t2766;
    let t19196 = t7389 * t2315;
    let t19204 = t672 * t818;
    let t19210 = t1087 * t2299;
    (t19161, t19179, t19196, t19204, t19210)
}
