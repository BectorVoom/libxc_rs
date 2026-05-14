//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 534/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk534<F: Float>(t1604: F, t7839: F, t1614: F, t408: F, t1608: F, t373: F, t1619: F, t397: F, t428: F, t1618: F, t388: F, t401: F, t409: F, t12: F, t52: F, t1703: F) -> (F, F, F, F, F, F, F) {
    let t7840 = t1604 * t7839;
    let t7843 = t408 * t1614;
    let t7845 = t1608 * t7843 * t373;
    let t7847 = t1619 * t397 * t428;
    let t7848 = t1618 * t7847;
    let t7852 = t388 * t409 * t401;
    let t7853 = t52 * t12;
    let t7854 = t7853 * t1703;
    (t7840, t7843, t7845, t7848, t7852, t7853, t7854)
}
