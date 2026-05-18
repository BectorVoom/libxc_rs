//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 764/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk764<F: Float>(t11913: F, t11962: F, t11966: F, t11969: F, t11973: F, t11977: F, t11981: F, t11984: F, t11990: F, t11994: F, t1901: F, t446: F, t8499: F, t8516: F, t8523: F, t8526: F, t8534: F) -> F {
    let t11997 = -t11913 - F::new(2.0) / F::new(27.0) * t8499 + F::new(8.0) / F::new(27.0) * t8516 + t8523 / F::new(9.0) + t8526 / F::new(27.0) - t8534 - t446 * t11962 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t11966 + F::new(2.0) / F::new(3.0) * t446 * t11969 - t446 * t11973 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t11977 - t11981 - F::new(2.0) / F::new(27.0) * t1901 * t11984 - F::new(10.0) / F::new(81.0) * t1901 * t11990 + t1901 * t11994 / F::new(9.0);
    t11997
}
