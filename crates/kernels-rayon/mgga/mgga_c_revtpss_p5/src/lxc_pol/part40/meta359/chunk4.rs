//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1247/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1247(t2747: f64, t2754: f64, t4450: f64, t4364: f64, t4365: f64, t231: f64, t2394: f64, t10770: f64, t2719: f64, t820: f64, t844: f64, t4368: f64) -> (f64, f64, f64, f64) {
    let t14910 = t2747 * t4450 * t2754;
    let t14914 = t4364 * t4365 * t2754;
    let t14917 = t231 * t2394;
    let t14919 = t10770 * t4365 * t14917;
    let t14923 = t820 * t2719 * t844;
    let t14925 = 0.40015750243531754508e-2_f64 * t14923 * t4368;
    (t14910, t14914, t14919, t14925)
}
