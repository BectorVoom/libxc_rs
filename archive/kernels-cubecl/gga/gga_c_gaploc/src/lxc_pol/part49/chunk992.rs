//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 992/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk992<F: Float>(t7064: F, t7069: F, t8878: F, t161: F, t1841: F, t2576: F, t33137: F, t13212: F, t7129: F, t10789: F, t2508: F, t2586: F) -> (F, F, F, F) {
    let t43042 = t7064 * t8878 * t7069;
    let t43043 = F::cast_from(0.1922631557535556071e-2_f64) * t43042;
    let t43046 = t1841 * t33137 * t161 * t2576;
    let t43049 = F::cast_from(0.23071578690426672851e-1_f64) * t7129 * t13212;
    let t43051 = t2508 * t10789 * t2586;
    (t43043, t43046, t43049, t43051)
}
