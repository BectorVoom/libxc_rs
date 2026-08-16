//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 861/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk861(t1347: f64, t1356: f64, t7019: f64, t3944: f64, t7002: f64, t3947: f64, t1345: f64, t1921: f64, t45: f64, t5590: f64, t6950: f64, t6952: f64, t6956: f64, t6988: f64, t6991: f64, t6997: f64, t7004: f64) -> (f64, f64, f64, f64) {
    let t7021 = t1347 * t7019 * t1356;
    let t7024 = t3944 * t7002;
    let t7025 = t7024 * t3947;
    let t7028 = -t6950 + t6952 - t6956 + t6988 + t6991 + 0.19751789702565206229e-1_f64 * t45 * t6997 - 0.11696446794910408142e1_f64 * t5590 * t1921 + 0.11696446794910408142e1_f64 * t1345 * t7004 - 0.58482233974552040708e0_f64 * t1345 * t7021 - 0.17315755899375863299e2_f64 * t1345 * t7025;
    (t7021, t7024, t7025, t7028)
}
