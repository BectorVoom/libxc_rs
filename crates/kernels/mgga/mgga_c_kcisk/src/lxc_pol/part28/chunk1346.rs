//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1346/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1346<F: Float>(t110815: F, t110817: F, t110907: F, t110912: F, t111524: F, t111533: F, t1628: F, t2356: F, t2360: F, t25277: F, t2776: F, t34005: F, t34656: F, t34662: F, t34670: F, t35536: F, t564: F, t566: F, t9295: F, t9636: F, t9904: F) -> (F,) {
    let t120975 = t9904 * t34005 / 8.0 - t2776 * t566 * t25277 / 16.0 + t110815 - t110817 + t111524 + t110907 + t2356 * t34662 / 8.0 - t110912 + t111533 - t35536 * t9636 / 8.0 - t2776 * t1628 * t9295 / 16.0 - t564 * t2360 * t34656 / 8.0 + t9904 * t34670 / 8.0;
    (t120975,)
}
