//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1032/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1032<F: Float>(t121346: F, t121363: F, t116: F, t32608: F, t25081: F, t8697: F, t32782: F, t571: F, t2110: F, t7337: F, t2045: F, t7541: F, t1464: F, t8720: F, t2118: F, t7318: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122504 = 0.37645955677973955999e-5 * t121346;
    let t122512 = 0.35702867204846465857e-4 * t121363;
    let t122570 = t32608 * t116;
    let t122647 = t8697 * t25081;
    let t122710 = t571 * t32782;
    let t122712 = t2110 * t7337;
    let t122714 = t7541 * t2045;
    let t122720 = t8720 * t1464;
    let t122722 = t7318 * t2118;
    (t122504, t122512, t122570, t122647, t122710, t122712, t122714, t122720, t122722)
}
