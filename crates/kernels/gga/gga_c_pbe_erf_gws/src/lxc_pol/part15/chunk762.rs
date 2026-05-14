//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 762/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk762<F: Float>(t6560: F, t6562: F, t346: F, t6158: F, t5: F, t6161: F, t337: F, t2121: F, t2100: F, t274: F, t2251: F, t2299: F, t2276: F, t22: F, t4258: F, t191: F) -> (F, F, F, F, F, F, F) {
    let t6563 = t6560 * t6562;
    let t6566 = t6158 * t346;
    let t6568 = t5 * t6161;
    let t6569 = t337 * t6568;
    let t6570 = t2121 * t6569;
    let t6573 = t274 * t2100;
    let t6578 = t2251 * t2299;
    let t6579 = t2276 * t6578;
    let t6587 = 1.0 / t22 / t4258;
    let t6588 = t6587 * t191;
    (t6563, t6566, t6569, t6570, t6573, t6579, t6588)
}
