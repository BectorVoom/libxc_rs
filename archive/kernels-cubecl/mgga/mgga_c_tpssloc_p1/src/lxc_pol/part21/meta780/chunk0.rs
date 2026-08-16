//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2706/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706<F: Float>(t19815: F, t3789: F, t40159: F, t6390: F, t236: F, t240: F, t3869: F, t247: F, t5249: F, t3798: F, t1354: F, t40130: F) -> (F, F, F, F, F, F, F) {
    let t57033 = t19815 * t3789;
    let t57041 = t40159 * t6390;
    let t57043 = t236 * t240;
    let t57044 = t57043 * t3869;
    let t57046 = t247 * t5249;
    let t57056 = t19815 * t3798;
    let t57057 = t57056 * t1354;
    let t57071 = t40130 * t6390;
    (t57033, t57041, t57043, t57044, t57046, t57057, t57071)
}
