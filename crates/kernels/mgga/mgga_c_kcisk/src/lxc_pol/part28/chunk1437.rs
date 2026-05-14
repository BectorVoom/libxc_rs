//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1437/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1437<F: Float>(t2642: F, t33197: F, t7261: F, t7644: F, t34411: F, t9990: F, t112576: F, t113123: F, t118355: F, t121787: F, t121796: F, t122527: F, t122759: F, t123072: F, t1775: F, t22592: F, t33196: F, t33208: F, t33297: F, t34395: F, t34412: F, t34419: F, t35395: F, t9740: F, t9741: F, t9743: F) -> (F,) {
    let t123085 = t7261 * t33197 * t2642 * t7644;
    let t123088 = t9990 * t34411;
    let t123114 = -0.10416666666666666667e-1 * t9740 * t123085 + 0.92592592592592592593e-2 * t123088 * t9743 - 0.60312500000000000001e-2 * t33196 * t122527 - 0.116403125e-2 * t34419 * t122527 - 0.40208333333333333334e-2 * t33196 * t123085 - 0.40208333333333333334e-2 * t33196 * t123072 - 0.18518518518518518519e-1 * t34412 * t34395 - 0.17361111111111111111e-2 * t33297 * t35395 - 0.17361111111111111111e-2 * t33208 * t35395 - 0.17361111111111111111e-2 * t9740 * t1775 * t9741 * t22592 - 0.77382407407407407407e-3 * t121787 - t118355 - 0.17411041666666666666e-2 * t121796 - 0.38691203703703703703e-3 * t112576 - 0.40208333333333333334e-2 * t113123 * t122759;
    (t123114,)
}
