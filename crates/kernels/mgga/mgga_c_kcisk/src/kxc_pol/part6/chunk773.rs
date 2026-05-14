//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 773/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk773<F: Float>(t10649: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F, t1646: F, t1659: F, t28385: F, t26: F, t10738: F, t16389: F, t22698: F, t22705: F, t22707: F, t28362: F) -> (F, F, F, F) {
    let t28393 = -t10649 - 4.0 / 9.0 * t15989 + 2.0 / 9.0 * t22564 - 2.0 / 3.0 * t22575 + t22583 / 3.0 - 10.0 / 27.0 * t28371 + 4.0 / 3.0 * t28375 - 2.0 / 3.0 * t28379 - 2.0 * t28383 + 2.0 * t28387 - t28391 / 3.0;
    let t28394 = t1646 * t28393;
    let t28403 = t1659 * t28385;
    let t28404 = t26 * t28403;
    let t28408 = -0.39862222222222222223e0 * t15989 + 0.46074375e0 * t28362 + 0.1898925e1 * t28394 - t10738 - 0.27385555555555555556e0 * t16389 + 0.5477111111111111111e-1 * t22698 + 0.19931111111111111111e0 * t22564 - 0.59793333333333333333e0 * t22575 + 0.29896666666666666667e0 * t22583 - 0.32862666666666666666e0 * t22705 + 0.16431333333333333333e0 * t22707 + 0.49293999999999999999e0 * t28404 - 0.59793333333333333333e0 * t28379 + 0.17938e1 * t28387;
    (t28393, t28394, t28404, t28408)
}
