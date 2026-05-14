//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 568/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk568<F: Float>(t1165: F, t1532: F, t930: F, t530: F, t1162: F, t4198: F, t4199: F, t945: F, t1541: F, t3375: F, t1545: F, t3379: F, t2450: F, t3402: F, t1090: F, t1181: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4443 = t1165 * t1532 * t930;
    let t4447 = t1165 * t530 * t930;
    let t4450 = t4198 * t1162;
    let t4452 = t1165 * t1532 * t4199;
    let t4456 = t1165 * t1532 * t945;
    let t4459 = t3375 * t1541;
    let t4462 = 0.17149607247227894789e-2 * t3379 * t1545;
    let t4463 = t2450 * t3402;
    let t4465 = t1181 * t530 * t1090;
    (t4443, t4447, t4450, t4452, t4456, t4459, t4462, t4463, t4465)
}
