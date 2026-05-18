//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 973/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk973<F: Float>(t12808: F, t16092: F, t16105: F, t16117: F, t16119: F, t16122: F, t16124: F, t16126: F, t16226: F, t16244: F, t16251: F, t16253: F, t16256: F, t17789: F, t17826: F, t17898: F, t17919: F, t187: F, t1921: F, t3921: F, t3940: F, t3948: F, t6125: F) -> F {
    let t17933 = t187 * (t17789 + t17826 + t17898 + t17919) - t16092 - F::new(0.58482233974552040708e0) * t6125 * t3940 - F::new(0.17315755899375863299e2) * t6125 * t3948 + F::new(0.19751789702565206229e-1) * t187 * t16244 - t16105 + t16117 + t16119 + t16122 + t16124 + t16126 + t16226 + F::new(0.11696446794910408142e1) * t6125 * t3921 - F::new(0.58482233974552040708e0) * t12808 * t1921 - t16251 + t16253 - t16256;
    t17933
}
