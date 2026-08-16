//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 992/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk992<F: Float>(t39708: F, t39711: F, t39715: F, t39717: F, t39721: F, t39723: F, t39728: F, t39732: F, t39737: F, t39741: F, t39744: F, t39747: F, t39753: F, t39757: F, t39761: F) -> F {
    let t40627 = -F::cast_from(4.0_f64) * t39708 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39711 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t39715 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39717 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39721 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t39723 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39728 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t39732 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39737 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39741 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t39744 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t39747 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39753 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t39757 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39761;
    t40627
}
