//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 384/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk384<F: Float>(t2075: F, t526: F, t27: F, t89: F, t1957: F, t1960: F, t1963: F, t1967: F, t1972: F, t1977: F, t1981: F, t1989: F) -> (F, F, F) {
    let t2076 = t526 * t2075;
    let t2078 = t89 * t27 * t2076;
    let t2080 = t1957 + t1960 + t1963 - t1967 / F::cast_from(27.0_f64) + t1972 / F::cast_from(9.0_f64) + t1977 / F::cast_from(9.0_f64) - t1981 / F::cast_from(18.0_f64) + t1989 / F::cast_from(3.0_f64) - t2078 / F::cast_from(6.0_f64);
    (t2076, t2078, t2080)
}
