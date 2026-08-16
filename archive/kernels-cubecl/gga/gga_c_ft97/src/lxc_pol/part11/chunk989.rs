//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 989/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk989<F: Float>(t40312: F, t40315: F, t40497: F, t40500: F, t40503: F, t40512: F, t40297: F, t40301: F, t40306: F, t40309: F, t40318: F, t40321: F, t40506: F, t40509: F) -> F {
    let t40575 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t40312;
    let t40576 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t40315;
    let t40579 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t40497;
    let t40580 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t40500;
    let t40581 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t40503;
    let t40584 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t40512;
    let t40585 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t40297 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t40301 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40306 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40309 - t40575 - t40576 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t40318 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t40321 + t40579 + t40580 - t40581 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t40506 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40509 + t40584;
    t40585
}
