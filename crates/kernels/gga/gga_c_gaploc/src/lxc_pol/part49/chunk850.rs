//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 850/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk850<F: Float>(t1: F, t1559: F, t544: F, t986: F, t10241: F, t1359: F, t1352: F, t3690: F, t3689: F, t447: F, t2366: F, t475: F) -> (F, F, F, F, F, F) {
    let t35204 = t544 * t1559 * t986 * t1;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t38267 = t3690 * t1352;
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    (t35204, t35215, t35216, t38267, t38272, t38276)
}
