//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1100/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1100<F: Float>(t143158: F, t152722: F, t33820: F, t143101: F, t143120: F, t143123: F, t152715: F, t152719: F, t152724: F, t152727: F, t152730: F, t152734: F, t152738: F, t152742: F, t152746: F, t152750: F, t152754: F, t152758: F) -> (F, F) {
    let t152760 = t33820 * t143158 * t152722;
    let t152765 = -t152715 / F::cast_from(3.0_f64) + t152719 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152724 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152727 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t152730 + F::cast_from(4.0_f64) * t152734 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152738 + t152742 / F::cast_from(2.0_f64) + t152746 / F::cast_from(2.0_f64) - t152750 / F::cast_from(3.0_f64) + t152754 / F::cast_from(4.0_f64) - t152758 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152760 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t143101 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t143120 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t143123;
    (t152760, t152765)
}
