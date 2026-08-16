//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1066/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1066<F: Float>(t2728: F, t3459: F, t5559: F, t10800: F, t1960: F, t3511: F, t11125: F, t977: F, t2595: F, t33992: F, t13241: F, t841: F) -> (F, F, F, F, F, F) {
    let t44207 = F::cast_from(12.0_f64) * t5559 * t3459 * t2728;
    let t44208 = t10800 * t2728;
    let t44211 = t1960 * t3511 * t2728;
    let t44215 = t1960 * t11125 * t977;
    let t44217 = t33992 * t2595;
    let t44221 = F::cast_from(6.0_f64) * t5559 * t13241 * t841;
    (t44207, t44208, t44211, t44215, t44217, t44221)
}
