//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 610/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk610<F: Float>(t12506: F, t2365: F, t1429: F, t901: F, t9302: F, t9298: F, t161: F, t165: F, t3085: F) -> (F, F, F, F, F) {
    let t12507 = t2365 * t12506;
    let t12508 = t1429 * t12507;
    let t12510 = t9302 * t901;
    let t12512 = t9298 * t901;
    let t12526 = t161 * t165 * t3085;
    (t12507, t12508, t12510, t12512, t12526)
}
