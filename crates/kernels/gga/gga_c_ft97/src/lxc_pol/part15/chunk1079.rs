//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1079/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1079<F: Float>(t64516: F, t78362: F, t78364: F, t78366: F, t78368: F, t78396: F, t87060: F, t87063: F, t87067: F, t87071: F, t87074: F, t87077: F, t87080: F, t87084: F) -> F {
    let t87214 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t78362 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78364 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78366 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t78368 + t64516 + t87060 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t87063 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t87067 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t78396 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t87071 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t87074 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t87077 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t87080 + F::cast_from(4.0_f64) * t87084;
    t87214
}
