//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1208/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1208<F: Float>(t70142: F, t83606: F, t89772: F, t89775: F, t89781: F, t89785: F, t89789: F, t89794: F, t89798: F, t89802: F, t89807: F, t89811: F, t89815: F) -> F {
    let t91158 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t89772 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t89775 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89781 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89785 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89789 - t70142 - t89794 / F::cast_from(18.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89798 - t89802 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t83606 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89807 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t89811 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89815;
    t91158
}
