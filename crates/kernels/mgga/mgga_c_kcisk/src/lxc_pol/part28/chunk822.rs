//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 822/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk822<F: Float>(t10024: F, t10028: F, t10039: F, t2042: F, t240: F, t2666: F, t2815: F, t5532: F, t7656: F, t802: F, t9760: F, t9964: F, t9965: F, t9966: F, t9969: F, t9989: F) -> (F,) {
    let t10043 = t9964 - t9965 - t9966 + t9969 - t9989 + t240 * (t10024 * t802 + 2.0 * t10028 * t5532 - t10039 * t2042 - t2666 * t9760 - t2815 * t7656 - t9964 + t9965 + t9966 - t9969 + t9989);
    (t10043,)
}
