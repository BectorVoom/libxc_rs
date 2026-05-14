//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 771/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk771<F: Float>(t36700: F, t44777: F, t549: F, t44712: F, t739: F, t1991: F, t590: F, t43389: F, t5241: F, t5640: F, t43400: F, t43403: F, t43407: F, t2617: F, t3621: F, t7803: F) -> (F, F, F, F, F, F, F, F) {
    let t45183 = 0.47667319935800568892e0 * t36700 * t549 * t44777;
    let t45184 = t739 * t44712;
    let t45187 = 0.1022478025437886658e1 * t1991 * t45184 * t590;
    let t45188 = 0.14570311862489884877e2 * t43389;
    let t45192 = 0.15337170381568299871e1 * t5640 * t5241 * t44712 * t590;
    let t45193 = 0.30674340763136599742e1 * t43400;
    let t45194 = 0.20705180015117204825e2 * t43403;
    let t45195 = 0.92023022289409799224e1 * t43407;
    let t45197 = t7803 * t3621 * t2617;
    (t45183, t45187, t45188, t45192, t45193, t45194, t45195, t45197)
}
