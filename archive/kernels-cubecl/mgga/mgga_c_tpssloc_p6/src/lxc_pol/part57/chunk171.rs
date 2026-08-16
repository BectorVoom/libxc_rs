//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 171/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk171<F: Float>(t59: F, t625: F, t40: F, t73: F, t52: F, t76: F, t111: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t626 = t59 * t625;
    let t627 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t626;
    let t632 = t40 * t40;
    let t634 = F::cast_from(1.0_f64) / t73 / t632;
    let t636 = t52 * t52;
    let t638 = F::cast_from(1.0_f64) / t76 / t636;
    let t652 = t89 * t111;
    (t626, t627, t632, t634, t636, t638, t652)
}
