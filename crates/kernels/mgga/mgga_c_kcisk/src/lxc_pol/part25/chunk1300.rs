//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1300/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1300<F: Float>(t17182: F, t34136: F, t9664: F, t34181: F, t9649: F, t415: F, t5175: F, t9956: F, t112416: F, t112439: F, t112445: F, t116285: F, t32921: F, t32942: F, t32990: F, t33005: F, t33021: F, t34078: F, t34122: F, t34137: F, t63617: F, t64506: F, t7261: F) -> (F, F) {
    let t116620 = 0.13888888888888888889e-1 * t9664 * t17182 * t34136;
    let t116621 = t17182 * t34181;
    let t116623 = 0.69444444444444444446e-2 * t9664 * t116621;
    let t116625 = 0.26805555555555555556e-2 * t9649 * t116621;
    let t116639 = t415 * t5175 * t9956;
    let t116641 = 0.34722222222222222223e-2 * t112439 - 0.20833333333333333334e-1 * t34122 * t33005 + 0.26805555555555555556e-2 * t112445 - 0.24125000000000000001e-1 * t32921 * t34078 - 0.24125000000000000001e-1 * t9649 * t116285 - 0.46561250000000000002e-2 * t112416 * t34078 - t116620 - t116623 - t116625 - 0.41666666666666666668e-1 * t32942 * t34137 - 0.41666666666666666668e-1 * t32990 * t34137 - 0.41666666666666666668e-1 * t9664 * t7261 * t33021 * t63617 - 0.20833333333333333334e-1 * t9664 * t7261 * t33021 * t64506 - 0.66327777777777777776e-2 * t116639;
    (t116639, t116641)
}
