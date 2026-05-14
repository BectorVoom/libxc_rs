//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1263/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1263<F: Float>(t3805: F, t9691: F, t1333: F, t32898: F, t32984: F, t9660: F, t32962: F, t33049: F, t5074: F, t32974: F, t32969: F, t18325: F, t32920: F, t32981: F, t32977: F, t1871: F, t5175: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t112663 = t3805 * t9691;
    let t112665 = t1333 * t32898;
    let t112667 = t32984 * t9660;
    let t112669 = t32962 * t9660;
    let t112674 = t5074 * t33049;
    let t112683 = t1333 * t32974;
    let t112696 = t1333 * t32969;
    let t112709 = t32920 * t18325;
    let t112724 = t1333 * t32981;
    let t112726 = t1333 * t32977;
    let t112728 = t5175 * t1871;
    (t112663, t112665, t112667, t112669, t112674, t112683, t112696, t112709, t112724, t112726, t112728)
}
