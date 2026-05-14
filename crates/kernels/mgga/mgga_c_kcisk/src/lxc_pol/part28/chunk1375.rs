//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1375/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1375<F: Float>(t35229: F, t4998: F, t9664: F, t1799: F, t24040: F, t9679: F, t24044: F, t22331: F, t33017: F, t10886: F, t35162: F, t34107: F, t6668: F, t34173: F, t415: F, t7070: F) -> (F, F, F, F, F, F, F) {
    let t121699 = t9664 * t4998 * t35229;
    let t121702 = t1799 * t9679 * t24040;
    let t121705 = t1799 * t9679 * t24044;
    let t121708 = t1799 * t33017 * t22331;
    let t121712 = t9664 * t10886 * t35162;
    let t121715 = t1799 * t34107 * t6668;
    let t121724 = t415 * t34173 * t7070;
    (t121699, t121702, t121705, t121708, t121712, t121715, t121724)
}
