//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1403/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1403<F: Float>(t110907: F, t110912: F, t111533: F, t119713: F, t120890: F, t1624: F, t2070: F, t2709: F, t2752: F, t28192: F, t28193: F, t294: F, t297: F, t35060: F, t5586: F, t559: F, t6642: F, t7727: F, t7728: F, t9408: F, t9575: F, t9895: F) -> (F,) {
    let t120903 = t110907 - t294 * t28193 * t2752 / 16.0 - t110912 + t111533 - t2709 * t2070 * t6642 / 8.0 - t294 * t5586 * t9895 / 8.0 - t2709 * t7727 * t1624 / 16.0 - t294 * t297 * (t119713 + t120890) / 16.0 - t294 * t7728 * t9575 / 16.0 - t2709 * t28192 * t559 / 16.0 + t9408 * t35060 / 16.0;
    (t120903,)
}
