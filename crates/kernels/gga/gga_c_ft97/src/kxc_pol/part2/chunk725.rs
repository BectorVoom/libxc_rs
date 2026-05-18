//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 725/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk725<F: Float>(t11416: F, t11036: F, t11041: F, t11043: F, t11048: F, t11052: F, t11056: F, t11061: F, t11066: F, t11070: F, t11073: F, t11076: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t7771: F, t8190: F, t8195: F) -> F {
    let t11417 = F::new(2.0) / F::new(9.0) * t11416;
    let t11418 = t8195 / F::new(18.0) - t11036 / F::new(27.0) - t11041 - F::new(2.0) / F::new(81.0) * t11043 - t11048 / F::new(9.0) - t11052 / F::new(3.0) - t11056 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t11061 - t7771 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t11066 + t11070 - t11073 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t11076 - t8190 - t11395 / F::new(6.0) - F::new(2.0) / F::new(9.0) * t11399 + F::new(11.0) / F::new(27.0) * t11404 + t11408 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t11413 - t11417;
    t11418
}
