//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1090/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1090<F: Float>(t2724: F, t2747: F, t4450: F, t14676: F, t4364: F, t4366: F, t10811: F, t4452: F, t2754: F, t4365: F, t231: F, t2394: F, t10770: F, t2719: F, t820: F, t844: F) -> (F, F, F, F, F, F, F) {
    let t14900 = t2747 * t4450 * t2724;
    let t14904 = t4364 * t14676 * t4366;
    let t14907 = t10811 * t4452;
    let t14910 = t2747 * t4450 * t2754;
    let t14914 = t4364 * t4365 * t2754;
    let t14917 = t231 * t2394;
    let t14919 = t10770 * t4365 * t14917;
    let t14923 = t820 * t2719 * t844;
    (t14900, t14904, t14907, t14910, t14914, t14919, t14923)
}
