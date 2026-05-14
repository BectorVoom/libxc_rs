//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 776/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk776<F: Float>(t1131: F, t6555: F, t1021: F, t1092: F, t1768: F, t5026: F, t1774: F, t4999: F, t3262: F, t3263: F, t6272: F, t1662: F, t1670: F, t3269: F, t1727: F, t3274: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6556 = t1131 * t6555;
    let t6557 = t1021 * t6556;
    let t6558 = t1092 * t6557;
    let t6560 = t5026 * t1768;
    let t6561 = t1092 * t6560;
    let t6563 = t4999 * t1774;
    let t6564 = t1092 * t6563;
    let t6570 = t3262 * t3263 * t6272;
    let t6574 = t3269 * t1662 * t1670;
    let t6578 = t3274 * t1662 * t1727;
    (t6556, t6557, t6558, t6560, t6561, t6563, t6564, t6570, t6574, t6578)
}
