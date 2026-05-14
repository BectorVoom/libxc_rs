//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 856/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk856<F: Float>(t11371: F, t11461: F, t11531: F, t11607: F, t576: F, t932: F, t996: F, t3723: F, t787: F, t876: F, t1054: F, t125: F, t825: F, t919: F, t3209: F, t3254: F, t3739: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11609 = t11371 + t11461 + t11531 + t11607;
    let t11610 = t576 * t11609;
    let t11612 = t996 * t932;
    let t11613 = t3723 * t787;
    let t11614 = t11612 * t11613;
    let t11616 = t3723 * t876;
    let t11617 = t1054 * t11616;
    let t11619 = t825 * t125;
    let t11620 = t11619 * t919;
    let t11621 = t3209 * t11620;
    let t11623 = t3254 * t3739;
    (t11609, t11610, t11612, t11613, t11614, t11616, t11617, t11619, t11620, t11621, t11623)
}
