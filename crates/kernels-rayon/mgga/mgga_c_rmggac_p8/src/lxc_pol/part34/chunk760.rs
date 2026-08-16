//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 760/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk760(t21719: f64, t7248: f64, t8503: f64, t8507: f64, t9188: f64, t3352: f64, t8807: f64, t68386: f64, t9045: f64, t14125: f64, t21713: f64, t8811: f64) -> (f64, f64, f64, f64, f64) {
    let t73752 = t21719 * t7248 * t8503;
    let t73755 = t21719 * t9188 * t8507;
    let t73758 = t21719 * t3352 * t8807;
    let t73761 = t68386 * t3352 * t9045;
    let t73764 = t21713 * t14125 * t8811;
    (t73752, t73755, t73758, t73761, t73764)
}
