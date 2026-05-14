//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 707/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk707<F: Float>(t11050: F, t3621: F, t11059: F, t3613: F, t2282: F, t8680: F, t925: F, t1073: F, t2267: F, t1570: F, t2266: F, t643: F, t920: F, t363: F, t1557: F, t8654: F) -> (F, F, F, F, F, F, F) {
    let t12102 = t3621 * t11050;
    let t12104 = t3613 * t11059;
    let t12108 = t8680 * t925 * t2282;
    let t12112 = t8680 * t1073;
    let t12113 = t12112 * t2267;
    let t12116 = t2266 * t1570;
    let t12117 = t920 * t643;
    let t12118 = t12117 * t363;
    let t12119 = t12116 * t12118;
    let t12122 = t8654 * t1557;
    (t12102, t12104, t12108, t12113, t12118, t12119, t12122)
}
