//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 893/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk893<F: Float>(t17794: F, t9744: F, t446: F, t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F, t17768: F, t17773: F, t17778: F, t17782: F, t17787: F, t17792: F, t9701: F, t9735: F) -> (F, F) {
    let t17795 = t9744 * t17794;
    let t17796 = t446 * t17795;
    let t17799 = -t17720 / F::cast_from(27.0_f64) + t17724 / F::cast_from(18.0_f64) + t17729 / F::cast_from(9.0_f64) - t17734 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17738 - t17742 / F::cast_from(9.0_f64) - t17746 / F::cast_from(3.0_f64) - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t17751 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t17755 + t17759 / F::cast_from(9.0_f64) + t17763 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17768 + t17773 / F::cast_from(18.0_f64) - t17778 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t17782 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t9735 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17787 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17792 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17796 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9701;
    (t17796, t17799)
}
