//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2686/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2686<F: Float>(t1307: F, t16094: F, t54665: F, t686: F, t16095: F, t3719: F, t2559: F, t5194: F, t5198: F, t118: F, t16018: F, t3739: F, t794: F) -> (F, F, F, F) {
    let t54690 = t16094 * t686 * t54665 * t1307;
    let t54698 = t16094 * t686 * t16095 * t3719;
    let t54701 = t2559 * t5194 * t5198;
    let t54702 = F::cast_from(0.11666666666666666666e0_f64) * t54701;
    let t54705 = t3739 * t118 * t794 * t16018;
    (t54690, t54698, t54702, t54705)
}
