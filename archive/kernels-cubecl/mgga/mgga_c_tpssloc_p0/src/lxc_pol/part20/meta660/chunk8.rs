//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2472/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2472<F: Float>(t11094: F, t3213: F, t4696: F, t4700: F, t48734: F, t48736: F, t48738: F, t48741: F, t48744: F, t48747: F, t48750: F, t48753: F, t48755: F, t48759: F) -> F {
    let t50755 = F::cast_from(6.0_f64) * t11094 * t3213 * t4696 * t4700 - t48734 + t48736 + t48738 - t48741 - t48744 - t48747 - t48750 - t48753 - t48755 + t48759;
    t50755
}
