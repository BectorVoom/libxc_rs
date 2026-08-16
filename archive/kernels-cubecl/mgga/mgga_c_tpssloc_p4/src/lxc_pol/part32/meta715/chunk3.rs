//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2256/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2256<F: Float>(t5587: F, t81886: F, t23041: F, t5619: F, t16753: F, t6605: F, t815: F, t16928: F, t25084: F, t16851: F, t221: F, t87420: F) -> (F, F, F, F, F) {
    let t98796 = t81886 * t5587;
    let t98798 = t23041 * t5619;
    let t98801 = t6605 * t815 * t16753;
    let t98803 = t25084 * t16928;
    let t98808 = t87420 * t221 * t16851;
    (t98796, t98798, t98801, t98803, t98808)
}
