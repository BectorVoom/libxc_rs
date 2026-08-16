//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1526/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1526<F: Float>(t2747: F, t2754: F, t4450: F, t4364: F, t4365: F, t231: F, t2394: F, t10770: F, t2719: F, t820: F, t844: F, t4368: F) -> (F, F, F, F) {
    let t14910 = t2747 * t4450 * t2754;
    let t14914 = t4364 * t4365 * t2754;
    let t14917 = t231 * t2394;
    let t14919 = t10770 * t4365 * t14917;
    let t14923 = t820 * t2719 * t844;
    let t14925 = F::cast_from(0.40015750243531754508e-2_f64) * t14923 * t4368;
    (t14910, t14914, t14919, t14925)
}
