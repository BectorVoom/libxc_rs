//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 718/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk718<F: Float>(t7164: F, t779: F, t2564: F, t731: F, t2541: F, t5836: F, t5009: F, t883: F, t2562: F, t943: F, t5230: F, t732: F, t2553: F, t2060: F, t2558: F, t2559: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7165 = t779 * t7164;
    let t7168 = t731 * t2564;
    let t7170 = t2541 * t5836;
    let t7173 = t883 * t5009;
    let t7174 = t2562 * t7173;
    let t7175 = t943 * t7174;
    let t7177 = t883 * t5230;
    let t7178 = t732 * t7177;
    let t7179 = t2553 * t7178;
    let t7181 = t2060 * t2558;
    let t7182 = t943 * t7181;
    let t7184 = t731 * t2559;
    (t7165, t7168, t7170, t7173, t7175, t7177, t7179, t7182, t7184)
}
