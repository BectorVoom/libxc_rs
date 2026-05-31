//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 747/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk747<F: Float>(t9698: F, t9742: F, t9747: F, t9755: F, t9759: F, t9763: F, t9765: F, t9768: F, t9773: F, t9777: F, t9883: F, t9893: F, t9970: F) -> F {
    let t10119 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t9698;
    let t10120 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9768 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9755 + t9759 + t9763 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9765 - F::cast_from(2.0_f64) * t9773 - F::cast_from(2.0_f64) * t9777 + F::cast_from(2.0_f64) * t9742 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9747 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9883 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9893 + t9970 / F::cast_from(2.0_f64) - t10119;
    t10120
}
