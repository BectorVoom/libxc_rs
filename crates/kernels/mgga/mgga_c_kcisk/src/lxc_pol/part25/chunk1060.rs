//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1060/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1060<F: Float>(t18863: F, t18882: F, t18902: F, t18921: F, t2656: F, t5531: F, t2666: F, t5552: F, t12342: F, t17771: F, t17774: F, t17778: F, t17779: F, t17783: F, t2042: F, t5527: F, t5532: F, t5533: F, t7656: F, t7690: F) -> (F, F, F, F) {
    let t18923 = t18863 + t18882 + t18902 + t18921;
    let t18925 = t2656 * t5531;
    let t18928 = t2666 * t5552;
    let t18933 = -t12342 * t2666 - t18923 * t2042 + 2.0 * t18925 * t5533 + 2.0 * t18928 * t5532 - 2.0 * t5527 * t7690 - t5552 * t7656 - t17771 + t17774 + t17778 + t17779 + t17783;
    (t18923, t18925, t18928, t18933)
}
