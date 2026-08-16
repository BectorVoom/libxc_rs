//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 506/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk506(t19: f64, t27: f64, t521: f64, t14: f64, t22: f64, t583: f64, t588: f64, t1320: f64, t1333: f64, t123: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3857 = t19 * t27;
    let t3859 = 20.0_f64 * t3857 * t521;
    let t3860 = t14 * t22;
    let t3862 = 12.0_f64 * t3860 * t521;
    let t3863 = t583 * t588;
    let t3865 = 32.0_f64 * t3863 * t521;
    let t3867 = 8.0_f64 * t1320 * t1333;
    let t3869 = t520 * t123;
    (t3857, t3859, t3860, t3862, t3863, t3865, t3867, t3869)
}
