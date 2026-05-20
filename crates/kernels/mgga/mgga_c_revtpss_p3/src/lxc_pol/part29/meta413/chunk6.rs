//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1510/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1510<F: Float>(t1065: F, t4772: F, t906: F, t1042: F, t2858: F, t4823: F, t1469: F, t3059: F, t4872: F, t999: F, t247: F, t3116: F) -> (F, F, F, F, F) {
    let t16138 = t1065 * t4772;
    let t16139 = t16138 * t906;
    let t16140 = t1042 * t16139;
    let t16143 = t4823 * t2858;
    let t16144 = t1042 * t16143;
    let t16147 = t1469 * t3059;
    let t16148 = t4872 * t16147;
    let t16149 = t1042 * t16148;
    let t16152 = t4772 * t999;
    let t16154 = t247 * t3116 * t16152;
    (t16140, t16144, t16149, t16152, t16154)
}
