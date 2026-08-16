//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 819/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk819(t521: f64, t9342: f64, t14: f64, t588: f64, t2496: f64, t4038: f64, t123: f64, t1330: f64, t2630: f64, t2516: f64, t676: f64, t3869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9854 = 24.0_f64 * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = 144.0_f64 * t9856;
    let t9858 = t4038 * t2496;
    let t9859 = 0.51947577317044391276e2_f64 * t9858;
    let t9860 = t1330 * t123;
    let t9861 = t9860 * t2630;
    let t9862 = 0.32530743900905219526e-1_f64 * t9861;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1_f64 * t3869 * t9863;
    (t9854, t9857, t9859, t9862, t9863, t9865)
}
