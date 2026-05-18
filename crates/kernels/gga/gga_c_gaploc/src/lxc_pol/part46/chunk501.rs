//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 501/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk501<F: Float>(t1570: F, t3085: F, t1339: F, t475: F, t3158: F, t494: F, t3116: F, t555: F, t599: F) -> (F, F, F, F, F, F, F) {
    let t9181 = t1570 * t3085;
    let t9182 = t1339 * t475;
    let t9183 = t9181 * t9182;
    let t9186 = t3158 * t494;
    let t9189 = t555 * t3116;
    let t9190 = t9189 * t494;
    let t9193 = t599 * t3085;
    (t9181, t9182, t9183, t9186, t9189, t9190, t9193)
}
