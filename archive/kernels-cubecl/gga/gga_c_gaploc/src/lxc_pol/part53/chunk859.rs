//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 859/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk859<F: Float>(t2876: F, t9453: F, t3159: F, t12874: F, t4527: F, t4614: F, t204: F, t41749: F, t587: F, t41738: F, t6710: F, t6711: F) -> (F, F, F, F) {
    let t42296 = t2876 * t9453;
    let t42298 = F::cast_from(0.16683561977530199113e1_f64) * t3159 * t42296;
    let t42305 = F::cast_from(0.36809208915763919689e2_f64) * t4527 * t4614 * t12874;
    let t42309 = F::cast_from(0.18404604457881959845e2_f64) * t587 * t204 * t41749;
    let t42312 = F::cast_from(0.43710935587469654631e2_f64) * t6710 * t6711 * t41738;
    (t42298, t42305, t42309, t42312)
}
