//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1229/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1229<F: Float>(t33673: F, t33701: F, t1459: F, t2347: F, t9571: F, t2748: F, t6638: F, t33644: F, t33646: F, t33648: F, t33650: F, t33653: F, t33656: F, t33659: F, t33661: F, t33663: F, t33665: F, t33667: F, t33669: F, t33671: F) -> (F, F, F, F, F) {
    let t33702 = t33673 + t33701;
    let t33703 = t1459 * t33702;
    let t33705 = t9571 * t2347;
    let t33708 = t2748 * t6638;
    let t33728 = -0.9375e-1 * t33644 + 0.26979166666666666667e-1 * t33646 - 0.20234375e-1 * t33648 + 0.9375e-1 * t33650 - 0.4046875e-1 * t33653 - 0.89930555555555555557e-2 * t33656 - 0.9375e-1 * t33659 - 0.9375e-1 * t33661 - 0.25e0 * t33663 + 0.14388888888888888889e0 * t33665 + 0.20234375e-1 * t33667 + 0.26979166666666666667e-1 * t33669 - 0.16666666666666666667e0 * t33671;
    (t33702, t33703, t33705, t33708, t33728)
}
