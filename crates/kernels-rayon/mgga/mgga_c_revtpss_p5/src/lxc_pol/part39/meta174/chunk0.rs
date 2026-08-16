//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 765/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk765(t187: f64, t3850: f64, t2608: f64, t520: f64, t512: f64, t189: f64, t19: f64, t27: f64, t521: f64, t14: f64, t22: f64, t583: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3852 = 0.19751673498613801407e-1_f64 * t3850 * t187;
    let t3853 = t520 * t2608;
    let t3854 = t512 * t3853;
    let t3855 = t3850 * t189;
    let t3856 = t512 * t3855;
    let t3857 = t19 * t27;
    let t3859 = 20.0_f64 * t3857 * t521;
    let t3860 = t14 * t22;
    let t3862 = 12.0_f64 * t3860 * t521;
    let t3863 = t583 * t588;
    (t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860, t3862, t3863)
}
