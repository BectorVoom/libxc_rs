//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 418/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk418<F: Float>(t3630: F, t738: F, t270: F, t3438: F, t3445: F, t3604: F, t3617: F, t3622: F, t3627: F, t1052: F) -> (F, F, F) {
    let t3631 = t738 * t3630;
    let t3634 = F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3604 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3617 + F::cast_from(0.1281754371690370714e-2_f64) * t3438 - F::cast_from(0.23071578690426672851e-1_f64) * t270 * t3622 - F::cast_from(0.1281754371690370714e-2_f64) * t3445 + F::cast_from(0.15381052460284448567e-1_f64) * t270 * t3627 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3631;
    let t3638 = t1052 * t1052;
    (t3631, t3634, t3638)
}
