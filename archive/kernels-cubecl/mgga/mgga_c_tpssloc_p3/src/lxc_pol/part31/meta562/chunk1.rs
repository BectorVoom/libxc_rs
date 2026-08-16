//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1792/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792<F: Float>(t252: F, t4119: F, t22690: F, t7520: F, t81573: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t22893: F, t23164: F, t25306: F) -> (F, F, F, F, F) {
    let t87130 = t252 * t4119;
    let t87140 = t81573 * t22690 * t7520;
    let t87153 = t6562 * t794 * t25324;
    let t87155 = t23030 * t25258;
    let t87165 = t23164 * t22893 * t25306;
    (t87130, t87140, t87153, t87155, t87165)
}
