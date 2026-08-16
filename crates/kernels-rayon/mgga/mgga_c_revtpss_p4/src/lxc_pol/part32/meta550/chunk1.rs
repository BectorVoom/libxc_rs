//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1866/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1866(t94589: f64, t96279: f64, t26265: f64, t9686: f64, t2098: f64, t4075: f64, t786: f64, t2103: f64, t47567: f64, t26261: f64, t40270: f64, t10073: f64, t25920: f64, t26260: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96456 = t94589 * t96279;
    let t96460 = t26265 * t9686;
    let t96463 = t786 * t2098 * t4075;
    let t96473 = 0.81814717454467823679e-4_f64 * t47567 * t2103;
    let t96491 = 0.96373646535613327356e-3_f64 * t40270 * t26261;
    let t96503 = t10073 * t25920 * t26260;
    (t96456, t96460, t96463, t96473, t96491, t96503)
}
