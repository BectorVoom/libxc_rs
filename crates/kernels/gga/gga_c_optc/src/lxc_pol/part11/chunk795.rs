//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 795/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk795<F: Float>(t9701: F, t9703: F, t9705: F, t16336: F, t16337: F, t16339: F, t6465: F, t6477: F, t6750: F, t6753: F, t6771: F, t6811: F, t9707: F, t9715: F, t13111: F, t13114: F) -> (F, F, F, F, F, F, F, F) {
    let t16340 = 60.0 * t9701;
    let t16341 = 36.0 * t9703;
    let t16342 = 96.0 * t9705;
    let t16343 = -t6750 + t6753 + t6465 + t6771 + t16336 + t16337 + t16339 + t6811 + t6477 + t16340 + t16341 + t16342;
    let t16344 = 0.73246220147012639764e-3 * t9707;
    let t16345 = 24.0 * t9715;
    let t16346 = 3.0 * t13111;
    let t16347 = 0.54934665110259479823e-3 * t13114;
    (t16340, t16341, t16342, t16343, t16344, t16345, t16346, t16347)
}
