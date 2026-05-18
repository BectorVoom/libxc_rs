//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 764/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk764<F: Float>(t1: F, t35951: F, t544: F, t11425: F, t1397: F, t11264: F, t524: F, t11385: F, t540: F, t106: F, t11218: F, t192: F) -> (F, F, F, F, F) {
    let t37675 = t544 * t35951 * t1;
    let t37679 = t1397 * t11425;
    let t37777 = t524 * t11264;
    let t37956 = t11385 * t540;
    let t37965 = t11218 * t1 * t106 * t192;
    (t37675, t37679, t37777, t37956, t37965)
}
