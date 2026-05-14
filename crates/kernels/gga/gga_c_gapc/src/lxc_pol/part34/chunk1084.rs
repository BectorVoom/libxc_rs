//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1084/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1084<F: Float>(t11350: F, t9241: F, t11540: F, t424: F, t612: F, t11333: F, t5252: F, t1386: F, t3157: F, t3674: F, t11561: F, t8734: F, t116: F, t25110: F, t27145: F, t33781: F) -> (F, F, F, F, F, F) {
    let t35031 = t11350 * t9241;
    let t35034 = t424 * t612 * t11540;
    let t35036 = t5252 * t11333;
    let t35039 = t1386 * t3674 * t3157;
    let t35041 = t11561 * t8734;
    let t35045 = t116 * t33781 * t25110 * t27145;
    (t35031, t35034, t35036, t35039, t35041, t35045)
}
