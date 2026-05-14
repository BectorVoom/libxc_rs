//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 872/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk872<F: Float>(t3521: F, t5934: F, t5950: F, t5955: F, t425: F, t5684: F, t11313: F, t2218: F, t1390: F, t1428: F, t180: F, t3529: F, t479: F, t3532: F, t1433: F, t1354: F, t2083: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19380 = 0.19711289e-2 * t3521 * t5934;
    let t19386 = 0.19711289e-2 * t3521 * t5950;
    let t19388 = 0.26281718666666666666e-2 * t3521 * t5955;
    let t19399 = t425 * t5684;
    let t19404 = t11313 * t2218;
    let t19412 = t1428 * t1390;
    let t19418 = t180 * t479 * t3529;
    let t19419 = t1428 * t3532;
    let t19423 = t1433 * t3532;
    let t19434 = t1354 * t2083;
    (t19380, t19386, t19388, t19399, t19404, t19412, t19418, t19419, t19423, t19434)
}
