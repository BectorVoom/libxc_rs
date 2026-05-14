//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1004/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1004<F: Float>(t3145: F, t334: F, t368: F, t3153: F, t73: F, t246: F, t676: F, t1046: F, t1041: F, t1038: F, t3229: F, t1036: F, t1033: F, t3169: F, t3173: F, t2866: F, t914: F) -> (F, F, F, F, F, F, F) {
    let t11243 = 1.0 / t3145 / t368 / t334;
    let t11249 = t3153 * t73;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11266 = t3229 * t1038;
    let t11267 = t1036 * t11266;
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    let t11289 = t2866 * t914;
    (t11243, t11249, t11262, t11264, t11268, t11271, t11289)
}
