//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 490/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk490(t1330: f64, t72: f64, t757: f64, t525: f64, t527: f64, t2608: f64, t520: f64, t512: f64, t19: f64, t27: f64, t521: f64, t14: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3833 = 1.0_f64 / t525;
    let t3841 = 1.0_f64 / t527;
    let t3853 = t520 * t2608;
    let t3854 = t512 * t3853;
    let t3857 = t19 * t27;
    let t3859 = 20.0_f64 * t3857 * t521;
    let t3860 = t14 * t22;
    (t3826, t3833, t3841, t3854, t3859, t3860)
}
