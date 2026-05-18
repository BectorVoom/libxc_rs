//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1078/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1078<F: Float>(t1094: F, t6480: F, t1122: F, t1092: F, t6708: F, t1134: F, t6487: F, t9532: F, t13192: F, t4807: F, t2825: F, t6629: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t18458 = t6480 * t1094;
    let t18459 = t18458 * sigma0;
    let t18460 = t18459 * t1122;
    let t18461 = t1092 * t18460;
    let t18463 = t6708 * sigma0;
    let t18464 = t18463 * t1134;
    let t18465 = t1092 * t18464;
    let t18467 = t9532 * t6487;
    let t18468 = t1092 * t18467;
    let t18471 = t13192 * t4807;
    let t18473 = t2825 * t6629;
    (t18458, t18461, t18463, t18465, t18468, t18471, t18473)
}
