//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1310/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1310<F: Float>(t1322: F, t35843: F, t6204: F, t8063: F, t32069: F, t8059: F, t1339: F, t26504: F, t32045: F, t25338: F, t9461: F, t25343: F, t26849: F, t3759: F, t25972: F, t5600: F) -> (F, F, F, F, F, F, F) {
    let t118822 = t6204 * t35843 * t8063 * t1322;
    let t118827 = t6204 * t32069 * t8059 * t1322;
    let t118837 = t1339 * t32045 * t26504;
    let t118840 = t1339 * t9461 * t25338;
    let t118843 = t1339 * t9461 * t25343;
    let t118846 = t3759 * t9461 * t26849;
    let t118849 = t5600 * t9461 * t25972;
    (t118822, t118827, t118837, t118840, t118843, t118846, t118849)
}
