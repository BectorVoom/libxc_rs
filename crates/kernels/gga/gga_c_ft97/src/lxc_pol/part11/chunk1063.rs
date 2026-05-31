//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1063/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1063<F: Float>(t41891: F, t41895: F, t41898: F, t41901: F, t41905: F, t41909: F, t41915: F, t41918: F, t41922: F, t41925: F, t41927: F, t41932: F, t41935: F, t41938: F, t41942: F) -> F {
    let t42042 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41891 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t41895 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t41898 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41901 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41905 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t41909 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t41915 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t41918 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41922 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41925 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t41927 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41932 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41935 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t41938 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41942;
    t42042
}
