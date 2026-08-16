//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 758/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk758(t14125: f64, t68440: f64, t9045: f64, t21713: f64, t68422: f64, t9050: f64, t21719: f64, t7248: f64, t8503: f64, t8507: f64, t9188: f64, t3352: f64, t8807: f64) -> (f64, f64, f64, f64, f64) {
    let t73746 = t68440 * t14125 * t9045;
    let t73749 = t21713 * t68422 * t9050;
    let t73752 = t21719 * t7248 * t8503;
    let t73755 = t21719 * t9188 * t8507;
    let t73758 = t21719 * t3352 * t8807;
    (t73746, t73749, t73752, t73755, t73758)
}
