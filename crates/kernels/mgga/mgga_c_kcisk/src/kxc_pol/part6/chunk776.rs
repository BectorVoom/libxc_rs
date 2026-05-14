//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 776/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk776<F: Float>(t1645: F, t28439: F, t10757: F, t28357: F, t10755: F, t10761: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F) -> (F, F, F) {
    let t28441 = 1.0 * t1645 * t28439;
    let t28442 = t28357 * t10757;
    let t28444 = 0.51725014705706168417e3 * t10755 * t28442;
    let t28455 = -t10761 - 0.12361111111111111111e-1 * t15989 + 0.61805555555555555556e-2 * t22564 - 0.18541666666666666667e-1 * t22575 + 0.92708333333333333334e-2 * t22583 - 0.10300925925925925926e-1 * t28371 + 0.37083333333333333333e-1 * t28375 - 0.18541666666666666666e-1 * t28379 - 0.55625000000000000001e-1 * t28383 + 0.55625000000000000001e-1 * t28387 - 0.92708333333333333333e-2 * t28391;
    (t28441, t28444, t28455)
}
