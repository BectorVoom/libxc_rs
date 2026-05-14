//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1363/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1363<F: Float>(t1610: F, t35037: F, t114859: F, t118647: F, t118650: F, t118653: F, t118655: F, t118656: F, t118659: F, t118661: F, t118662: F, t118664: F, t118667: F, t118669: F, t1620: F, t21345: F, t22056: F, t28136: F, t33750: F, t4535: F, t57167: F, t6607: F, t8455: F, t9557: F, t9571: F, t9882: F, t9891: F) -> (F,) {
    let t119887 = t35037 * t1610;
    let t119901 = 2.0 * t4535 * t8455 * t9571 + 4.0 * t114859 * t6607 - t119887 * t1620 + 4.0 * t21345 * t33750 - 2.0 * t22056 * t9891 - t28136 * t9557 + 4.0 * t57167 * t9882 - t118647 - t118650 - t118653 - t118655 + t118656 + t118659 + t118661 + t118662 + t118664 - t118667 + t118669;
    (t119901,)
}
