//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 890/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk890<F: Float>(t44712: F, t5241: F, t5640: F, t590: F, t43400: F, t43403: F, t43407: F, t2617: F, t3621: F, t7803: F, t43412: F, t43416: F) -> (F, F, F, F, F, F, F) {
    let t45192 = F::new(0.15337170381568299871e1) * t5640 * t5241 * t44712 * t590;
    let t45193 = F::new(0.30674340763136599742e1) * t43400;
    let t45194 = F::new(0.20705180015117204825e2) * t43403;
    let t45195 = F::new(0.92023022289409799224e1) * t43407;
    let t45197 = t7803 * t3621 * t2617;
    let t45199 = F::new(0.15337170381568299871e1) * t43412;
    let t45200 = F::new(0.15337170381568299871e1) * t43416;
    (t45192, t45193, t45194, t45195, t45197, t45199, t45200)
}
