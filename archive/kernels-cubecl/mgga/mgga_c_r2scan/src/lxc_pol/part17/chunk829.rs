//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 829/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk829<F: Float>(t2591: F, t8740: F, t7337: F, t8735: F, t5109: F, t495: F, t7321: F, t2551: F, t3090: F, t2573: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8741 = t8740 * t2591;
    let t8742 = t7337 * t8741;
    let t8745 = t8735 * t2591;
    let t8746 = t5109 * t8745;
    let t8749 = t5109 * t8741;
    let t8752 = t8740 * t495;
    let t8753 = t7321 * t8752;
    let t8756 = t8740 * t2551;
    let t8757 = t7321 * t8756;
    let t8760 = t3090 * t495;
    let t8761 = t5109 * t8760;
    let t8764 = t8735 * t2573;
    (t8741, t8742, t8745, t8746, t8749, t8752, t8753, t8756, t8757, t8760, t8761, t8764)
}
