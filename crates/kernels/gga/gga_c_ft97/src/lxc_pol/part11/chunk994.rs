//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 994/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk994<F: Float>(t40301: F, t40306: F, t40309: F, t40312: F, t40315: F, t40318: F, t40321: F, t40490: F, t40494: F, t40497: F, t40500: F, t40503: F, t40506: F, t40509: F, t40512: F) -> F {
    let t40660 = -F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t40301 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40306 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40309 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t40312 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t40315 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t40318 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t40321 + t40490 / F::cast_from(6.0_f64) - t40494 / F::cast_from(4.0_f64) + F::cast_from(112.0_f64) / F::cast_from(243.0_f64) * t40497 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t40500 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t40503 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t40506 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40509 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t40512;
    t40660
}
