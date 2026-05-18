//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 760/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk760<F: Float>(t2925: F, t935: F, t7290: F, t10667: F, t296: F, t1022: F, t2530: F, t2958: F, t7291: F, t688: F, t123: F, t24884: F) -> (F, F, F, F, F, F, F) {
    let t32356 = t2925 * t935;
    let t32357 = t7290 * t32356;
    let t32364 = t296 * t10667;
    let t32435 = t1022 * t2530;
    let t32436 = t7290 * t32435;
    let t32607 = t2958 * t7291;
    let t32658 = t32435 * t688;
    let t32692 = t24884 * t123;
    (t32357, t32364, t32435, t32436, t32607, t32658, t32692)
}
