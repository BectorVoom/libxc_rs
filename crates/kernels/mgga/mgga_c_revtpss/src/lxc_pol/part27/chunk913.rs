//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 913/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk913<F: Float>(t11028: F, t780: F, t2439: F, t10910: F, t225: F, t2772: F, t779: F, t689: F, t781: F, t9292: F, t861: F, t867: F) -> (F, F, F, F, F) {
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    let t11032 = t10910 * t225;
    let t11036 = t779 * t2772;
    let t11037 = t689 * t11036;
    let t11040 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t781;
    let t11043 = t861 * t867;
    (t11030, t11032, t11037, t11040, t11043)
}
