//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1119/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1119<F: Float>(t1610: F, t1650: F, t27584: F, t4440: F, t4314: F, t531: F, t1615: F, t6159: F, t1444: F, t1616: F, t5654: F, t3754: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28747 = t1650 * t1610;
    let t28748 = t27584 * t28747;
    let t28749 = t4440 * t28748;
    let t28752 = t4314 * t531;
    let t28753 = t1650 * t1615;
    let t28754 = t28752 * t28753;
    let t28755 = t6159 * t28754;
    let t28758 = t1616 * t1444;
    let t28759 = t28758 * t5654;
    let t28760 = t6159 * t28759;
    let t28765 = t1616 * t3754;
    (t28748, t28749, t28752, t28754, t28755, t28758, t28759, t28760, t28765)
}
