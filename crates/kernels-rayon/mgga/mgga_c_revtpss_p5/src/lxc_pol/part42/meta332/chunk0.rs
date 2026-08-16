//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1127/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1127(t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64) -> (f64, f64, f64, f64, f64) {
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    let t14777 = t10811 * t4462;
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14783 = 7.0_f64 / 72.0_f64 * t2703 * t4458;
    (t14761, t14765, t14777, t14780, t14783)
}
