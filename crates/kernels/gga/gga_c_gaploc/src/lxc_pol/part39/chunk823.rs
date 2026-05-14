//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 823/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk823<F: Float>(t2876: F, t9453: F, t3159: F, t12874: F, t4527: F, t4614: F, t204: F, t41749: F, t587: F, t41738: F, t6710: F, t6711: F, t6717: F, t6914: F, t12943: F, t4379: F) -> (F, F, F, F, F, F) {
    let t42296 = t2876 * t9453;
    let t42298 = 0.16683561977530199113e1 * t3159 * t42296;
    let t42305 = 0.36809208915763919689e2 * t4527 * t4614 * t12874;
    let t42309 = 0.18404604457881959845e2 * t587 * t204 * t41749;
    let t42312 = 0.43710935587469654631e2 * t6710 * t6711 * t41738;
    let t42315 = 0.12423108009070322895e3 * t6914 * t6717 * t41749;
    let t42316 = t4379 * t12943;
    (t42298, t42305, t42309, t42312, t42315, t42316)
}
