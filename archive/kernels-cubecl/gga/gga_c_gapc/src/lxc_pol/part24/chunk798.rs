//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 798/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk798<F: Float>(t1074: F, t9532: F, t1018: F, t876: F, t3272: F, t1045: F, t7442: F, t1092: F, t2542: F, t3281: F, t7208: F, t906: F) -> (F, F, F, F, F, F) {
    let t9533 = t9532 * t1074;
    let t9535 = t1018 * t876;
    let t9536 = t3272 * t9535;
    let t9538 = t1045 * t7442;
    let t9539 = t1092 * t9538;
    let t9541 = t2542 * t3281;
    let t9543 = t7208 * t906;
    (t9533, t9536, t9538, t9539, t9541, t9543)
}
