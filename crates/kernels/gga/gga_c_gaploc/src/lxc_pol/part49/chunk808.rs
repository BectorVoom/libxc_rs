//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 808/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk808<F: Float>(t177: F, t208: F, t4347: F, t1397: F, t4390: F, t1238: F, t4072: F, t4081: F, t92: F, t153: F, t155: F, t4080: F) -> (F, F, F, F, F) {
    let t17293 = t177 / t4347 / t208;
    let t18067 = t1397 * t4390;
    let t18089 = F::cast_from(1.0_f64) / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    let t18096 = t153 / t4080 / t155;
    (t17293, t18067, t18089, t18091, t18096)
}
