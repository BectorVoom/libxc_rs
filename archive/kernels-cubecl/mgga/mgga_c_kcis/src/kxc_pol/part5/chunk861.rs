//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 861/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk861<F: Float>(t1347: F, t1356: F, t7019: F, t3944: F, t7002: F, t3947: F, t1345: F, t1921: F, t45: F, t5590: F, t6950: F, t6952: F, t6956: F, t6988: F, t6991: F, t6997: F, t7004: F) -> (F, F, F, F) {
    let t7021 = t1347 * t7019 * t1356;
    let t7024 = t3944 * t7002;
    let t7025 = t7024 * t3947;
    let t7028 = -t6950 + t6952 - t6956 + t6988 + t6991 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t6997 - F::cast_from(0.11696446794910408142e1_f64) * t5590 * t1921 + F::cast_from(0.11696446794910408142e1_f64) * t1345 * t7004 - F::cast_from(0.58482233974552040708e0_f64) * t1345 * t7021 - F::cast_from(0.17315755899375863299e2_f64) * t1345 * t7025;
    (t7021, t7024, t7025, t7028)
}
