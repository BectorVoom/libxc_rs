//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 733/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk733<F: Float>(t160: F, t19531: F, t4348: F, t892: F, t10523: F, t1422: F, t544: F, t1564: F, t165: F, t10524: F, t1415: F, t1433: F, t9271: F) -> (F, F, F, F, F, F) {
    let t19532 = t19531 * t160;
    let t19933 = t892 * t4348;
    let t20367 = t544 * t10523 * t1422;
    let t20368 = t165 * t1564;
    let t20471 = t1415 * t10524;
    let t20535 = t1433 * t9271;
    (t19532, t19933, t20367, t20368, t20471, t20535)
}
