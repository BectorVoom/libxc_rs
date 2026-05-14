//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 896/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk896<F: Float>(t11620: F, t3209: F, t3254: F, t3739: F, t1061: F, t6179: F, t2440: F, t3728: F, t2212: F, t2268: F, t3738: F, t10346: F, t2208: F, t6201: F, t800: F, t3649: F, t760: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11621 = t3209 * t11620;
    let t11623 = t3254 * t3739;
    let t11625 = t1061 * t6179;
    let t11626 = t3728 * t2440;
    let t11627 = t11625 * t11626;
    let t11629 = t2268 * t2212;
    let t11630 = t3738 * t11629;
    let t11632 = t10346 * t2208;
    let t11633 = t800 * t6201;
    let t11634 = t11632 * t11633;
    let t11636 = t3649 * t760;
    (t11621, t11623, t11625, t11626, t11627, t11629, t11630, t11632, t11633, t11634, t11636)
}
