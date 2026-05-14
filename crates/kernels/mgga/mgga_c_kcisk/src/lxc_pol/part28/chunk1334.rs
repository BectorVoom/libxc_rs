//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1334/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1334<F: Float>(t47649: F, t786: F, t2803: F, t60823: F, t79: F, t17182: F, t34405: F, t34419: F, t34473: F, t9739: F, t123: F, t2801: F, t34455: F, t34411: F, t9720: F, t112835: F) -> (F, F, F, F, F, F, F, F) {
    let t117671 = t786 * t47649;
    let t117683 = t60823 * t79 * t2803;
    let t117687 = t17182 * t34405;
    let t117688 = t34419 * t117687;
    let t117690 = t34473 * t9739;
    let t117694 = t2801 * t34455 * t123;
    let t117702 = t9720 * t34411;
    let t117705 = t112835 * t9739;
    (t117671, t117683, t117687, t117688, t117690, t117694, t117702, t117705)
}
