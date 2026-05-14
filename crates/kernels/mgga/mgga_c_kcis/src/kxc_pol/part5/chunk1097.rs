//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1097/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1097<F: Float>(t1219: F, t20505: F, t5233: F, t5237: F, t10865: F, t6788: F, t10862: F, t10884: F, t10898: F, t15369: F, t15460: F, t20475: F, t20479: F, t20486: F, t20489: F, t20492: F, t20495: F, t20498: F, t20502: F, t3550: F, t3575: F, t3585: F, t3592: F, t5216: F, t5238: F) -> (F,) {
    let t20506 = t20505 * t1219;
    let t20509 = t5237 * t5233;
    let t20512 = t6788 * t10865;
    let t20513 = t20512 * t1219;
    let t20516 = -0.11696446794910408142e1 * t3585 * t20475 + 0.17315755899375863299e2 * t3592 * t20479 - 4.0 * t15460 * t5216 + 0.64329366355741395948e2 * t15369 * t5238 + 6.0 * t3575 * t20486 - 4.0 * t3550 * t20489 - 0.19298809906722418785e3 * t10898 * t20492 - 2.0 * t3550 * t20495 + 0.34631511798751726598e2 * t3592 * t20498 + 0.1025389702100779493e4 * t10884 * t20502 + 0.32164683177870697974e2 * t3575 * t20506 + 0.64329366355741395948e2 * t3575 * t20509 + 0.20691336878655965246e4 * t10862 * t20513;
    (t20516,)
}
