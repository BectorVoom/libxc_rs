//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 640/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk640<F: Float>(t10714: F, t573: F, t10560: F, t4744: F, t10570: F, t10572: F, t10574: F, t10576: F, t10587: F, t10595: F, t10607: F, t10610: F, t10613: F, t10615: F, t10617: F, t10619: F, t10623: F, t10626: F) -> (F, F) {
    let t10715 = t573 * t10714;
    let t10716 = t10560 * t4744;
    let t10718 = 0.96490945932906628932e2 * t10715 * t10716;
    let t10733 = -0.32862666666666666666e0 * t10607 + 0.16431333333333333333e0 * t10610 - 0.49293999999999999999e0 * t10613 - 0.27385555555555555556e0 * t10615 + 0.16431333333333333333e0 * t10617 + 0.5477111111111111111e-1 * t10619 - 0.36514074074074074075e-1 * t10623 - 0.82156666666666666667e-1 * t10626 - 0.59793333333333333333e0 * t10587 + 0.17938e1 * t10595 - 0.39862222222222222223e0 * t10570 + 0.19931111111111111111e0 * t10572 - 0.59793333333333333333e0 * t10574 + 0.29896666666666666667e0 * t10576;
    (t10718, t10733)
}
