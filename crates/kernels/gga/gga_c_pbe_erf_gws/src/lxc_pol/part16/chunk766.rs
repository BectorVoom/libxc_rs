//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 766/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk766<F: Float>(t2118: F, t6638: F, t339: F, t911: F, t824: F, t822: F, t2157: F, t6177: F, t337: F, t2121: F, t2302: F, t2323: F, t56: F, t931: F, t19: F, t274: F, t6161: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6639 = t2118 * t6638;
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6645 = t822 * t6644;
    let t6646 = t6177 * t2157;
    let t6647 = t337 * t6646;
    let t6648 = t2121 * t6647;
    let t6656 = t2323 * t2302;
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6665 = t274 * t6161;
    (t6639, t6643, t6644, t6645, t6646, t6648, t6656, t6659, t6665)
}
