//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1051/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1051<F: Float>(t24112: F, t7310: F, t17933: F, t7440: F, t23980: F, t7430: F, t17846: F, t1941: F, t9016: F, t11763: F, t9047: F, t17975: F, t739: F, t7312: F, t5310: F, t9069: F) -> (F, F, F, F, F, F, F, F) {
    let t24113 = t7310 * t24112;
    let t24115 = t17933 * t7440;
    let t24117 = t7430 * t23980;
    let t24118 = t17846 * t24117;
    let t24120 = t9016 * t1941;
    let t24122 = t11763 * t9047;
    let t24125 = t739 * t17975;
    let t24126 = t24125 * t7312;
    let t24128 = t5310 * t9069;
    (t24113, t24115, t24117, t24118, t24120, t24122, t24126, t24128)
}
