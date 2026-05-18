//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1321/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1321<F: Float>(t10328: F, t11688: F, t23132: F, t24081: F, t17874: F, t35469: F, t11673: F, t128: F, t22970: F, t24499: F, t10346: F, t11683: F, t23305: F, t2440: F) -> (F, F, F, F, F) {
    let t35764 = t10328 * t11688;
    let t35766 = t24081 * t23132;
    let t35768 = t35766 * t35469 * t17874;
    let t35772 = t11673 * t22970 * t128 * t24499;
    let t35776 = t10346 * t23305 * t11683 * t2440;
    (t35764, t35766, t35768, t35772, t35776)
}
