//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 992/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk992<F: Float>(t3308: F, t6021: F, t165: F, t5397: F, t935: F, t161: F, t23000: F, t5241: F, t10012: F, t1710: F, t2684: F, t9438: F, t23099: F, t7396: F, t948: F, t10018: F, t7375: F) -> (F, F, F, F, F, F) {
    let t28099 = t6021 * t3308;
    let t28126 = t165 * t935 * t5397;
    let t28129 = 0.23005755572352449806e1 * t23000 * t5241 * t161 * t28126;
    let t28141 = t2684 * t9438 * t10012 * t1710;
    let t28150 = t23099 * t948 * t7396;
    let t28156 = t7375 * t10018;
    (t28099, t28126, t28129, t28141, t28150, t28156)
}
