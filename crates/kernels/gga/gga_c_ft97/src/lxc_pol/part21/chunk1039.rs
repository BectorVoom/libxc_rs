//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1039/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1039<F: Float>(t3392: F, t39801: F, t6: F, t8: F, t1642: F, t5778: F, t1348: F, t1771: F, t5775: F, t23400: F, t378: F, t2178: F, t5929: F, t458: F, t5889: F) -> (F, F, F, F, F, F, F) {
    let t94936 = t3392 * t39801 * t6 * t8;
    let t94976 = t1642 * t5778;
    let t94983 = t1348 * t1771;
    let t94984 = t94983 * t5775;
    let t95009 = t378 * t23400;
    let t95021 = t5929 * t2178;
    let t95053 = t5889 * t458;
    (t94936, t94976, t94983, t94984, t95009, t95021, t95053)
}
