//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 748/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk748<F: Float>(t10667: F, t325: F, t3431: F, t723: F, t7290: F, t701: F, t2610: F, t2925: F, t935: F, t296: F, t1022: F, t2530: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32179 = t325 * t10667;
    let t32214 = t3431 * t723;
    let t32215 = t7290 * t32214;
    let t32260 = t3431 * t701;
    let t32261 = t2610 * t32260;
    let t32356 = t2925 * t935;
    let t32357 = t7290 * t32356;
    let t32364 = t296 * t10667;
    let t32435 = t1022 * t2530;
    (t32179, t32214, t32215, t32260, t32261, t32356, t32357, t32364, t32435)
}
