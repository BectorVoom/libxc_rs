//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 949/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk949<F: Float>(t204: F, t41749: F, t587: F, t41738: F, t6710: F, t6711: F, t6717: F, t6914: F, t12943: F, t4379: F, t10608: F, t9272: F, t9278: F) -> (F, F, F, F, F) {
    let t42309 = F::cast_from(0.18404604457881959845e2_f64) * t587 * t204 * t41749;
    let t42312 = F::cast_from(0.43710935587469654631e2_f64) * t6710 * t6711 * t41738;
    let t42315 = F::cast_from(0.12423108009070322895e3_f64) * t6914 * t6717 * t41749;
    let t42316 = t4379 * t12943;
    let t42349 = t9272 * t10608 * t9278;
    (t42309, t42312, t42315, t42316, t42349)
}
