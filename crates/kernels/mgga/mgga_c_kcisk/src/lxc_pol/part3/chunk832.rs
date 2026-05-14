//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 832/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk832<F: Float>(t1254: F, t13674: F, t1232: F, t4079: F, t346: F, t360: F, t4082: F, t13589: F, t13522: F, t13526: F, t13530: F, t13533: F, t13536: F, t13540: F, t13543: F, t13546: F, t13549: F, t13552: F, t13555: F) -> (F, F, F, F) {
    let t13675 = t13674 * t1254;
    let t13679 = 1.0 / t4079 / t1232;
    let t13680 = t346 * t13679;
    let t13682 = 1.0 / t4082 / t360;
    let t13683 = t13589 * t13682;
    let t13686 = 0.28842592592592592592e-1 * t13522;
    let t13697 = -t13686 - 0.12361111111111111111e-1 * t13526 + 0.61805555555555555556e-2 * t13530 - 0.18541666666666666667e-1 * t13533 + 0.92708333333333333334e-2 * t13536 - 0.10300925925925925926e-1 * t13540 + 0.37083333333333333333e-1 * t13543 - 0.18541666666666666666e-1 * t13546 - 0.55625000000000000001e-1 * t13549 + 0.55625000000000000001e-1 * t13552 - 0.92708333333333333333e-2 * t13555;
    (t13675, t13680, t13683, t13697)
}
