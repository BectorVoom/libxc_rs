//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1479/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1479<F: Float>(t10662: F, t913: F, t2842: F, t2860: F, t919: F, t2862: F, t931: F) -> (F, F, F, F) {
    let t10737 = t10662 * t913;
    let t10739 = F::cast_from(6.0_f64) * t2842 * t10737;
    let t10740 = t919 * t2860;
    let t10743 = t2862 * t931;
    (t10737, t10739, t10740, t10743)
}
