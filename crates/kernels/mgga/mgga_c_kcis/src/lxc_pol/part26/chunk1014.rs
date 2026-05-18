//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1014/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1014<F: Float>(t187: F, t6996: F, t1357: F, t1585: F, t21172: F, t21174: F, t21176: F, t21178: F, t21180: F, t21293: F, t21295: F, t21320: F, t21324: F, t21330: F, t21334: F, t21449: F, t22842: F, t22884: F, t22927: F, t22983: F, t4381: F, t5615: F, t6125: F, t7004: F, t7025: F) -> F {
    let t22989 = t187 * t6996;
    let t23006 = t187 * (t22842 + t22884 + t22927 + t22983) + t21172 + t21174 + t21176 - t21178 + t21180 + t21293 + t21295 + F::new(0.11696446794910408142e1) * t4381 * t7004 - F::new(0.58482233974552040708e0) * t22989 * t1357 - F::new(0.58482233974552040708e0) * t1585 * t21449 - F::new(0.17315755899375863299e2) * t4381 * t7025 - F::new(0.11696446794910408142e1) * t6125 * t5615 - F::new(0.17315755899375863299e2) * t1585 * t21334 - F::new(0.34631511798751726598e2) * t1585 * t21330 + F::new(0.1038945353962551798e3) * t1585 * t21320 + F::new(0.11696446794910408142e1) * t1585 * t21324;
    t23006
}
