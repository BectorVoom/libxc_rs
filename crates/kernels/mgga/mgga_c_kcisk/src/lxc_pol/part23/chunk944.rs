//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 944/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk944<F: Float>(t1428: F, t19144: F, t457: F, t1417: F, t5938: F, t2083: F, t3558: F, t3540: F, t12848: F, t3545: F, t1471: F, t442: F, t460: F, t2222: F, t3517: F, t3559: F) -> (F, F, F, F, F, F, F, F) {
    let t19145 = t1428 * t19144;
    let t19146 = t457 * t19145;
    let t19150 = 0.19711289e-2 * t1417 * t5938;
    let t19151 = t3558 * t2083;
    let t19152 = t19151 * t3540;
    let t19155 = t12848 * t2083;
    let t19156 = t19155 * t3545;
    let t19160 = t1471 * t460 * t442;
    let t19163 = t3517 * t2222;
    let t19165 = t2083 * t3559;
    (t19145, t19146, t19150, t19152, t19156, t19160, t19163, t19165)
}
