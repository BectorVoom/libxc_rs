//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 952/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk952<F: Float>(t1054: F, t11616: F, t125: F, t825: F, t919: F, t3209: F, t3254: F, t3739: F, t1061: F, t6179: F, t2440: F, t3728: F) -> (F, F, F, F, F, F, F) {
    let t11617 = t1054 * t11616;
    let t11619 = t825 * t125;
    let t11620 = t11619 * t919;
    let t11621 = t3209 * t11620;
    let t11623 = t3254 * t3739;
    let t11625 = t1061 * t6179;
    let t11626 = t3728 * t2440;
    (t11617, t11619, t11620, t11621, t11623, t11625, t11626)
}
