//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1170/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1170<F: Float>(t1882: F, t28116: F, t6927: F, t8232: F, t28291: F, t8392: F, t10007: F, t109322: F, t109490: F, t109611: F, t109807: F, t1131: F, t1456: F, t1901: F, t2409: F, t242: F, t24852: F, t265: F, t28187: F, t28255: F, t3281: F, t4005: F, t42575: F, t446: F, t6061: F, t668: F, t684: F, t6861: F, t724: F, t729: F, t98051: F, t98053: F) -> (F,) {
    let t111405 = 2.0 / 9.0 * t1882 * t28116;
    let t111420 = t8232 * t6927;
    let t111436 = 2.0 / 27.0 * t8392 * t28291;
    let t111440 = -2.0 / 3.0 * t446 * t729 * t4005 * t6061 - t446 * t729 * t24852 * t1131 / 3.0 + t111405 - t446 * t242 * t109807 / 3.0 - 2.0 / 9.0 * t1901 * t42575 * t28187 + 2.0 / 9.0 * t1901 * t10007 * t6861 * t2409 - 2.0 / 9.0 * t1901 * t10007 * t28255 * t684 - 4.0 / 27.0 * t111420 - t446 * t242 * t109490 / 3.0 + 2.0 / 9.0 * t3281 * t724 * t1456 * t668 - t446 * t729 * t265 * t109322 / 3.0 + 4.0 / 27.0 * t98051 - 4.0 / 9.0 * t98053 - t111436 + 4.0 / 3.0 * t446 * t242 * t109611;
    (t111440,)
}
