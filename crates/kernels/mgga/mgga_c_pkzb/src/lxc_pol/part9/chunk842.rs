//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 842/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk842<F: Float>(t405: F, t6460: F, t394: F, t5728: F, t758: F, t5939: F, t922: F, t918: F, t2029: F, t2387: F, t3207: F, t406: F, t54: F, t931: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6461 = t405 * t6460;
    let t6462 = t5728 * t394;
    let t6463 = t6461 * t6462;
    let t6464 = t758 * t6463;
    let t6467 = t5939 * t922;
    let t6468 = t918 * t6467;
    let t6470 = t2387 * t2029;
    let t6471 = t6470 * t3207;
    let t6472 = t406 * t6471;
    let t6475 = t54 * t931;
    (t6461, t6462, t6463, t6464, t6467, t6468, t6471, t6472, t6475)
}
