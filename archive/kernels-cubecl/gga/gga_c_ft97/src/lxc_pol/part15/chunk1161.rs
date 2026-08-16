//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1161/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1161<F: Float>(t41951: F, t80770: F, t80772: F, t80819: F, t80821: F, t88221: F, t88225: F, t88229: F, t89027: F, t89030: F, t89034: F, t89038: F, t89042: F, t89047: F) -> F {
    let t89727 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t80770 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t80772 + t41951 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t88221 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t88225 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t88229 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t80819 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t80821 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t89027 - F::cast_from(2.0_f64) * t89030 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89034 - t89038 / F::cast_from(9.0_f64) + t89042 / F::cast_from(3.0_f64) - F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t89047;
    t89727
}
