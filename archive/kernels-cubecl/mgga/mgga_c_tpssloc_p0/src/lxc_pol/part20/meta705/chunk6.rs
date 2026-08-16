//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2684/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2684<F: Float>(t16118: F, t9577: F, t212: F, t5187: F, t12225: F, t2586: F, t16100: F, t782: F, t16103: F, t16081: F, t16090: F, t16093: F, t16097: F, t2566: F) -> (F, F, F, F, F, F) {
    let t54663 = t9577 * t16118;
    let t54665 = t212 * t5187;
    let t54667 = t2586 * t12225 * t54665;
    let t54668 = F::cast_from(0.49999999999999999998e-2_f64) * t54667;
    let t54670 = t782 * t16100;
    let t54671 = t54670 * t16103;
    let t54673 = t16081 * t16090;
    let t54676 = t2566 * t16093 * t16097;
    (t54663, t54665, t54668, t54671, t54673, t54676)
}
