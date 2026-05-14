//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 385/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk385<F: Float>(t3358: F, t492: F, t105: F, t3124: F, t3132: F, t3329: F, t3341: F, t3346: F, t3349: F, t3353: F, t3357: F, t1016: F, t921: F, t2877: F, t895: F, t189: F, t3338: F) -> (F, F, F, F, F) {
    let t3359 = t492 * t3358;
    let t3362 = t3329 + 0.28455006635676149599e-1 * t105 * t3341 + t3346 - t3349 + t3124 - t3132 - t3353 + t3357 - 0.28455006635676149599e-1 * t105 * t3359;
    let t3366 = t1016 * t921;
    let t3370 = 0.35750489951850426669e0 * t895 * t2877;
    let t3371 = t189 * t3338;
    (t3359, t3362, t3366, t3370, t3371)
}
