//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1038/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1038<F: Float>(t1210: F, t8381: F, t8378: F, t3725: F, t7819: F, t1536: F, t8350: F, t2293: F, t6540: F, t1203: F, t7796: F, t5788: F, t6560: F, t14736: F, t14810: F, t1543: F, t21742: F, t21764: F, t21869: F, t2297: F, t4436: F, t4461: F, t4468: F, t4471: F, t4478: F, t6523: F, t6545: F, t6549: F, t6557: F, t8375: F) -> (F,) {
    let t27607 = t8381 * t1210;
    let t27610 = t8378 * t1210;
    let t27613 = t7819 * t3725;
    let t27614 = t27613 * t1210;
    let t27621 = t8350 * t1536;
    let t27624 = t2293 * t6540;
    let t27627 = t7796 * t1203;
    let t27638 = t6560 * t5788;
    let t27641 = -0.1038945353962551798e3 * t14736 * t27607 - 0.11696446794910408142e1 * t4471 * t27610 + 0.17315755899375863299e2 * t4478 * t27614 - 4.0 * t21869 * t6523 + 0.64329366355741395948e2 * t21764 * t6545 + 6.0 * t4461 * t27621 - 4.0 * t4436 * t27624 + 0.58482233974552040708e0 * t27627 * t1543 + 0.11696446794910408142e1 * t21742 * t2297 + 0.11696446794910408142e1 * t6549 * t6557 - 0.11696446794910408142e1 * t14810 * t8375 + 0.58482233974552040708e0 * t4468 * t8378 + 0.34631511798751726598e2 * t4478 * t27638;
    (t27641,)
}
