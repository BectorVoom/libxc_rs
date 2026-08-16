//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 813/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk813(t1056: f64, t6334: f64, t345: f64, t6326: f64, t1030: f64, t104: f64, t1072: f64, t3105: f64, t3109: f64, t3113: f64, t4869: f64, t4871: f64, t4885: f64, t4887: f64, t6276: f64) -> (f64, f64, f64) {
    let t6436 = t1056 * t6334;
    let t6439 = t345 * t6326;
    let t6450 = t3105 - t3109 - t3113 - 0.3513e-2_f64 * t104 * t6436 + 0.1171e-2_f64 * t104 * t6439 + 0.11955719325063177623e-1_f64 * t1030 * t6276 - 0.5179538907796306876e-4_f64 * t1072 * t6276 - 0.23911438650126355246e-1_f64 * t4869 + 0.20718155631185227504e-3_f64 * t4871 - 0.26416666666666666666e-2_f64 * t4885 - 0.23526125e-4_f64 * t4887;
    (t6436, t6439, t6450)
}
