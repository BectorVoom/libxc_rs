//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 199/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk199<F: Float>(t397: F, t658: F, t786: F, t782: F, t722: F, t772: F, t737: F, t749: F, t240: F, t753: F, t157: F, t32: F, t5: F, t28: F, t14: F, t15: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t788 = t397 * t658 * t786;
    let t791 = 1.0 + 0.2698618307426597582e-1 * t782 * t788;
    let t792 = f64::ln(t791);
    let t794 = 1.0 + 0.193e0 * t792;
    let t795 = 1.0 / t794;
    let t798 = t772 * t795 + 0.17411041666666666666e-2 * t722;
    let t801 = 1.0 + 0.9375e-1 * t737 - 0.101171875e-1 * t749;
    let t802 = 1.0 / t801;
    let t806 = t753 + t240 * (t798 * t802 - t753);
    let t812 = 0.11073577833333333333e-2 * t5 * t157 * t32;
    let t813 = t28 * t28;
    let t814 = 1.0 / t813;
    let t815 = t14 * t814;
    let t816 = 1.0 / t15;
    (t788, t791, t794, t795, t798, t801, t802, t806, t812, t813, t814, t815, t816)
}
