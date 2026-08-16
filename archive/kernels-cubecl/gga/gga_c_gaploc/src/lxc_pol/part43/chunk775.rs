//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 775/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk775<F: Float>(t10241: F, t1359: F, t544: F, t1352: F, t3690: F, t3689: F, t447: F, t2366: F, t475: F, t6508: F, t12000: F, t158: F) -> (F, F, F, F, F, F, F) {
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t38267 = t3690 * t1352;
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    let t38277 = t6508 * t38276;
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    (t35215, t35216, t38267, t38272, t38277, t38281, t38285)
}
