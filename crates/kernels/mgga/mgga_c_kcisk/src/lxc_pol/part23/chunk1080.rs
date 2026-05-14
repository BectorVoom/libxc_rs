//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1080/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1080<F: Float>(t240: F, t5761: F, t1213: F, t14850: F, t1550: F, t19456: F, t19459: F, t19478: F, t19480: F, t19590: F, t19594: F, t19693: F, t19699: F, t19703: F, t2107: F, t3699: F, t3718: F, t3726: F, t4486: F, t5771: F, t5790: F, t5795: F, t6568: F) -> (F,) {
    let t21710 = t240 * t5761;
    let t21713 = 0.1038945353962551798e3 * t1550 * t19699 + 0.23392893589820816284e1 * t4486 * t5771 + 0.23392893589820816284e1 * t1550 * t19703 - 0.17315755899375863299e2 * t1550 * t19594 - 0.34631511798751726598e2 * t4486 * t5795 - 0.1025389702100779493e4 * t1550 * t19590 + 0.11696446794910408142e1 * t1550 * t19456 - 0.11696446794910408142e1 * t4486 * t5790 - 0.58482233974552040708e0 * t6568 * t3718 + 0.11696446794910408142e1 * t6568 * t3699 - 0.17315755899375863299e2 * t6568 * t3726 - 0.58482233974552040708e0 * t14850 * t2107 - 0.35089340384731224426e1 * t1550 * t19459 - 0.58482233974552040708e0 * t1550 * t19693 - 0.11696446794910408142e1 * t21710 * t1213 + t19478 + t19480;
    (t21713,)
}
