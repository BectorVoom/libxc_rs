//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 654/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk654<F: Float>(t304: F, t330: F, t5557: F, t123: F, t2084: F, t160: F, t4348: F, t498: F, t177: F, t208: F, t4347: F, t1397: F, t4390: F, t1238: F, t4072: F, t4081: F, t92: F) -> (F, F, F, F, F, F, F, F) {
    let t16710 = t304 / t5557 / t330;
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t17288 = t498 * t4348;
    let t17293 = t177 / t4347 / t208;
    let t18067 = t1397 * t4390;
    let t18089 = 1.0 / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    (t16710, t16879, t16880, t17288, t17293, t18067, t18089, t18091)
}
