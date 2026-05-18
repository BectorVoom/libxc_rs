//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 522/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk522<F: Float>(t1773: F, t1776: F, t1778: F, t3125: F, t3128: F, t3131: F, t3135: F, t3139: F, t3141: F, t3144: F, t3146: F, t3151: F, t3155: F, t462: F, t92: F) -> F {
    let t3157 = t1773 + t1776 / F::new(9.0) + t1778 / F::new(3.0) + t3125 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t462 * t3128 + t462 * t3131 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t462 * t3135 - F::new(2.0) / F::new(3.0) * t3139 * t3141 + t3144 / F::new(3.0) + t462 * t3146 / F::new(3.0) + F::new(2.0) * t462 * t3151 - t92 * t3155;
    t3157
}
