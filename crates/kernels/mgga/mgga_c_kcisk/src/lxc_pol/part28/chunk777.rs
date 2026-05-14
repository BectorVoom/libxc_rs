//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 777/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk777<F: Float>(t1895: F, t9687: F, t415: F, t1900: F, t717: F, t2785: F, t9645: F, t9649: F, t9652: F, t9657: F, t9662: F, t9664: F, t9667: F, t9672: F, t9678: F, t9682: F, t9685: F) -> (F, F, F, F, F) {
    let t9688 = t9687 * t1895;
    let t9689 = t415 * t9688;
    let t9691 = t717 * t1900;
    let t9692 = t415 * t9691;
    let t9694 = -0.10416666666666666667e-1 * t9645 * t2785 + 0.40208333333333333335e-2 * t9649 * t9652 - 0.10416666666666666667e-1 * t9657 * t2785 - t9662 - 0.34722222222222222223e-2 * t9664 * t9667 + 0.10416666666666666667e-1 * t9664 * t9672 + 0.10416666666666666667e-1 * t9664 * t9652 + t9678 + 0.16581944444444444444e-2 * t9682 + 0.24872916666666666666e-2 * t9685 - 0.24872916666666666666e-2 * t9689 + 0.16581944444444444444e-2 * t9692;
    (t9688, t9689, t9691, t9692, t9694)
}
