//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 292/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk292<F: Float>(t474: F, t1023: F, t1049: F, t484: F, t481: F, t1107: F, t496: F, t492: F, t490: F, t176: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1188 = 1.0 / t474;
    let t1192 = 0.19388333333333333333e1 * t1023;
    let t1194 = 0.12315e-2 * t1049;
    let t1198 = t484 * t484;
    let t1199 = 1.0 / t1198;
    let t1200 = t481 * t1199;
    let t1201 = 0.72691666666666666667e3 * t1023;
    let t1203 = 0.78666666666666666667e2 * t1049;
    let t1213 = t1107 * t496;
    let t1214 = t492 * t1213;
    let t1216 = t490 * t1214 / 6.0;
    let t1217 = t176 * t487;
    (t1188, t1192, t1194, t1198, t1199, t1200, t1201, t1203, t1213, t1214, t1216, t1217)
}
