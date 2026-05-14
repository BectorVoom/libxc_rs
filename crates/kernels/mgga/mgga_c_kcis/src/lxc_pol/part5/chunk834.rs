//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 834/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk834<F: Float>(t609: F, t1585: F, t187: F, t1921: F, t6125: F, t6950: F, t6952: F, t6956: F, t6988: F, t6991: F, t6997: F, t7004: F, t7021: F, t7025: F, t7478: F, t1608: F, t286: F) -> (F, F, F, F) {
    let t614 = 0.0 < t609;
    let t7490 = -t6950 + t6952 - t6956 + t6988 + t6991 + t187 * t7478 + 0.19751789702565206229e-1 * t187 * t6997 - 0.11696446794910408142e1 * t6125 * t1921 + 0.11696446794910408142e1 * t1585 * t7004 - 0.58482233974552040708e0 * t1585 * t7021 - 0.17315755899375863299e2 * t1585 * t7025;
    let t7492 = piecewise3(t614, t7490, -t7490);
    let t7493 = t1608 * t7492;
    let t7494 = t286 * t7493;
    (t7490, t7492, t7493, t7494)
}
