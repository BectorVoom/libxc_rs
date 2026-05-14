//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 514/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk514<F: Float>(t2269: F, t362: F, t2263: F, t1428: F, t176: F, t998: F) -> (F, F, F, F) {
    let t4039 = t362 * t2269;
    let t4044 = t362 * t2263;
    let t4053 = t176 * t1428;
    let t4054 = t4053 * t998;
    (t4039, t4044, t4053, t4054)
}
