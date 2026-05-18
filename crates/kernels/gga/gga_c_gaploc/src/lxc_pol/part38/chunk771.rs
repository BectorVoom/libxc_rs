//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 771/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk771<F: Float>(t11425: F, t1397: F, t11264: F, t524: F, t11385: F, t540: F, t1: F, t106: F, t11218: F, t192: F, t3516: F, t594: F) -> (F, F, F, F, F) {
    let t37679 = t1397 * t11425;
    let t37777 = t524 * t11264;
    let t37956 = t11385 * t540;
    let t37965 = t11218 * t1 * t106 * t192;
    let t37975 = t594 * t3516;
    (t37679, t37777, t37956, t37965, t37975)
}
