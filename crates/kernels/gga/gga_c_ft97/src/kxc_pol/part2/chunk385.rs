//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 385/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk385<F: Float>(t2080: F, t515: F, t152: F, t153: F, t590: F, t91: F, t151: F, t1771: F, t1775: F, t583: F, t458: F, t588: F) -> (F, F, F, F, F, F, F) {
    let t2081 = t515 * t2080;
    let t2086 = F::cast_from(1.0_f64) / t153 / t152;
    let t2087 = t590 * t590;
    let t2089 = t91 * t2086 * t2087;
    let t2092 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1771 * t151;
    let t2093 = t1775 * t583;
    let t2095 = t458 * t588;
    (t2081, t2086, t2087, t2089, t2092, t2093, t2095)
}
