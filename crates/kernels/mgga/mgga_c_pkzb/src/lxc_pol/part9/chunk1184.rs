//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1184/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1184<F: Float>(t2196: F, t3030: F, t2199: F, t1171: F, t6141: F, t6144: F, t2256: F, t3080: F, t1189: F, t6312: F, t2240: F, t2242: F, t8003: F, t851: F, t2234: F, t8198: F) -> (F, F, F, F, F, F) {
    let t22820 = t3030 * t2196;
    let t22822 = 6.0 * t22820 * t2199;
    let t22823 = t1171 * t6141;
    let t22825 = 0.96491876992155210402e2 * t22823 * t6144;
    let t22826 = t3080 * t2256;
    let t22829 = t1189 * t6312;
    let t22837 = 0.48245938496077605201e2 * t2240 * t8003 * t2242 * t851;
    let t22840 = 0.48245938496077605201e2 * t2240 * t8198 * t2234;
    (t22822, t22825, t22826, t22829, t22837, t22840)
}
