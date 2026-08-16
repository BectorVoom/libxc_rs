//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1582/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1582<F: Float>(t135: F, t3471: F, t1174: F, t11168: F, t4908: F, t11159: F, t4900: F, t1184: F, t4899: F) -> (F, F, F, F, F) {
    let t11560 = t135 * t3471;
    let t11561 = t1174 * t11560;
    let t11563 = t4908 * t11168;
    let t11566 = t4900 * t11159;
    let t11569 = t4899 * t1184;
    (t11560, t11561, t11563, t11566, t11569)
}
