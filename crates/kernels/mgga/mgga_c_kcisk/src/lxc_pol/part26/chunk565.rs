//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 565/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk565<F: Float>(t1342: F, t5606: F, t1339: F, t2178: F, t3748: F, t2075: F, t3764: F, t1340: F, t1446: F, t2213: F, t415: F, t1286: F, t2059: F, t3485: F, t3484: F, t3482: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5607 = t5606 * t1342;
    let t5608 = t1339 * t5607;
    let t5610 = t3748 * t2178;
    let t5612 = t3764 * t2075;
    let t5613 = t1340 * t5612;
    let t5614 = t1339 * t5613;
    let t5616 = t2213 * t1446;
    let t5617 = t415 * t5616;
    let t5620 = t2059 * t1286;
    let t5621 = t3485 * t5620;
    let t5622 = t3484 * t5621;
    let t5623 = t3482 * t5622;
    (t5607, t5608, t5610, t5612, t5613, t5614, t5616, t5617, t5621, t5622, t5623)
}
