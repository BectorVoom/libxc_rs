//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 657/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk657<F: Float>(t6059: F, t769: F, t121: F, t5745: F, t2084: F, t321: F, t2088: F, t324: F, t304: F, t330: F, t5557: F, t123: F, t160: F, t4348: F, t498: F, t177: F, t208: F, t4347: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15766 = t769 * t6059;
    let t16534 = t121 * t5745;
    let t16687 = t2084 * t321;
    let t16692 = 1.0 / t2088 / t324;
    let t16710 = t304 / t5557 / t330;
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t17288 = t498 * t4348;
    let t17293 = t177 / t4347 / t208;
    (t15766, t16534, t16687, t16692, t16710, t16879, t16880, t17288, t17293)
}
