//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 532/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk532(t1064: f64, t4581: f64, t4567: f64, t945: f64, t1079: f64, t1056: f64, t345: f64, t104: f64, t111: f64, t120: f64, t3061: f64, t3105: f64, t3109: f64, t3113: f64, t3114: f64, t3122: f64, t3130: f64, t3150: f64, t4547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4898 = t1064 * t4581;
    let t4901 = t945 * t4567;
    let t4904 = t1079 * t4581;
    let t4907 = t1056 * t4567;
    let t4910 = t1056 * t4581;
    let t4913 = t345 * t4567;
    let t4919 = -0.23911438650126355246e-1_f64 * t3061 * t4547 + 0.15538616723388920628e-3_f64 * t3150 * t4547 - 0.1585e-2_f64 * t111 * t4898 - 0.52833333333333333333e-3_f64 * t111 * t4901 - 0.10082625e-4_f64 * t120 * t4904 - 0.672175e-5_f64 * t120 * t4907 + 0.7026e-2_f64 * t104 * t4910 + 0.1171e-2_f64 * t104 * t4913 + t3105 - t3109 - t3113 + 0.4684e-2_f64 * t3114 - 0.13208333333333333333e-2_f64 * t3122 - 0.117630625e-4_f64 * t3130;
    (t4898, t4901, t4904, t4907, t4910, t4913, t4919)
}
